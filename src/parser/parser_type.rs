use crate::ast::element::Element;
use crate::ast::markdown_doc::MarkdownDoc;
use crate::ast::message_type::MessageType;
use crate::ast::parsed_markdown::ParsedMarkdown;
use crate::ast::tokenized_markdown::TokenizedMarkdown;
use crate::parser::constants::FRONTMATTER_TOPICS_MAX;
use crate::parser::parse_result::ParseResult;
use crate::parser::parse_error_type::ParseErrorType;
use crate::parser::parse_error::ParseError;
use crate::parser::platforms::zeta::frontmatter::ZetaFrontmatter;
use crate::r#macro::ParsedMacro;
use crate::token::token::Token;
use crate::token::token_type::TokenType;

pub struct Parser {
    source: Vec<Token>,
    frontmatter: String,

    position: usize,

    nesting_levels: Vec<usize>,

    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(md: TokenizedMarkdown) -> Self {
        Self {
            source: md.elements,
            frontmatter: md.frontmatter,
            position: 0,
            nesting_levels: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn parse(mut self) -> std::result::Result<ParsedMarkdown, Vec<ParseError>> {
        let frontmatter = match self.parse_frontmatter() {
            Ok(frontmatter) => frontmatter,
            Err(error) => {
                self.errors.push(error);
                ZetaFrontmatter::default()
            }
        };

        let elements = self.parse_body()?;

        Ok(ParsedMarkdown {
            elements,
            frontmatter,
        })
    }

    fn parse_frontmatter(&mut self) -> ParseResult<ZetaFrontmatter> {
        let content = &self.frontmatter;

        let result = serde_yaml::from_str::<ZetaFrontmatter>(content).map_err(|error| {
            let (row, col) = if let Some(location) = error.location() {
                (location.line(), location.column())
            } else {
                (0, 0)
            };

            ParseError::new(ParseErrorType::InvalidFrontMatter, row, col)
        });

        if let Ok(frontmatter) = &result {
            if frontmatter.topics.len() > FRONTMATTER_TOPICS_MAX {
                return Err(ParseError::new(
                    ParseErrorType::TooManyTopics(frontmatter.topics.clone()),
                    0,
                    0,
                ));
            }
        }

        result
    }

    fn parse_body(mut self) -> std::result::Result<Vec<Element>, Vec<ParseError>> {
        let elements = self.parse_block(None);
        if !self.errors.is_empty() {
            return Err(self.errors);
        }
        Ok(elements)
    }

    fn parse_block(&mut self, end: Option<TokenType>) -> Vec<Element> {
        let mut elements = Vec::new();

        while let Some(token) = self.peek() {
            if let Some(ref end) = end {
                if token.token_type == *end {
                    break;
                }
            }

            let element = match self.parse_element() {
                Ok(element) => element,
                Err(error) => {
                    self.errors.push(error);
                    break;
                }
            };

            elements.push(element);
        }

        if let Some(end) = end {
            if self.peek().is_none() {
                self.errors.push(ParseError::new(
                    ParseErrorType::CouldNotFindEndToken(end),
                    0,
                    0
                ));
            }
        }

        elements
    }

    fn parse_element(&mut self) -> ParseResult<Element> {
        let Some(token) = self.advance().cloned() else {
            unreachable!("parse_element() should not be called when source is empty");
        };

        let elem = match token.token_type {
            TokenType::Text(text) => Element::Text(text),
            TokenType::Url(url) => Element::Url(url),
            TokenType::Image { alt, url } => Element::Image { alt, url },
            TokenType::LinkCard { card_type, url } => Element::LinkCard { card_type, url },
            TokenType::InlineFootnote(footnote) => Element::InlineFootnote(footnote),
            TokenType::Footnote(footnote) => Element::Footnote(footnote),
            TokenType::MessageBegin { level, r#type } => {
                let msg_type = match r#type.as_str() {
                    "info" => MessageType::Info,
                    "warn" => MessageType::Warn,
                    "alert" => MessageType::Alert,
                    _ => {
                        return Err(ParseError::new(
                            ParseErrorType::InvalidMessageType,
                            token.row,
                            token.col,
                        ))
                    }
                };
                self.nest(level, token.row, token.col)?;
                let body = self.parse_block(Some(TokenType::MessageOrDetailsEnd { level }));
                self.advance();
                self.unnest();
                Element::Message {
                    level,
                    msg_type,
                    body,
                }
            }
            TokenType::DetailsBegin { level, title } => {
                self.nest(level, token.row, token.col)?;
                let body = self.parse_block(Some(TokenType::MessageOrDetailsEnd { level }));
                self.advance();
                self.unnest();
                Element::Details { level, title, body }
            }
            TokenType::MessageOrDetailsEnd { level: _ } => Element::Text("".to_string()),
            TokenType::Macro(macro_info) => {
                let zenn_parser = Parser::new(MarkdownDoc {
                    frontmatter: String::new(),
                    elements: macro_info.zenn,
                });
                let zenn_elements = match zenn_parser.parse_body() {
                    Ok(zenn_elements) => zenn_elements,
                    Err(errors) => {
                        self.errors.extend(errors);
                        return Err(ParseError::new(
                            ParseErrorType::InvalidMacro,
                            token.row,
                            token.col,
                        ));
                    }
                };

                let qiita_parser = Parser::new(MarkdownDoc {
                    frontmatter: String::new(),
                    elements: macro_info.qiita,
                });
                let qiita_elements = match qiita_parser.parse_body() {
                    Ok(qiita_elements) => qiita_elements,
                    Err(errors) => {
                        self.errors.extend(errors);
                        return Err(ParseError::new(
                            ParseErrorType::InvalidMacro,
                            token.row,
                            token.col,
                        ));
                    }
                };

                Element::Macro(ParsedMacro {
                    zenn: zenn_elements,
                    qiita: qiita_elements,
                })
            }
        };

        Ok(elem)
    }

    fn advance(&mut self) -> Option<&Token> {
        let result = self.source.get(self.position);
        self.position += 1;
        result
    }

    fn peek(&mut self) -> Option<&Token> {
        self.source.get(self.position)
    }

    fn nest(&mut self, level: usize, row: usize, col: usize) -> ParseResult<()> {
        if let Some(last) = self.nesting_levels.last() {
            if level >= *last {
                return Err(ParseError::new(
                    ParseErrorType::InvalidNestingLevel(level),
                    row,
                    col,
                ));
            }
        }

        self.nesting_levels.push(level);

        Ok(())
    }

    fn unnest(&mut self) {
        self.nesting_levels
            .pop()
            .expect("unnest() should be called only when nesting_levels is not empty");
    }
}
