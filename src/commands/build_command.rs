use std::path::PathBuf;
use std::fs;
use std::fs::DirBuilder;
use serde::Deserialize;
use crate::parser::platforms::qiita::compiler::QiitaCompiler;
use crate::parser;
use crate::ast::parsed_markdown::ParsedMarkdown;
use crate::constants::ENV_QIITA_DIRECTORY;
use crate::parser::platforms::qiita::frontmatter::QiitaFrontmatter;
use crate::parser::platforms::platform::PlatformType;
use crate::parser::platforms::qiita::util::get_qiita_file;
use crate::parser::platforms::zenn::compiler::ZennCompiler;
use crate::parser::platforms::zenn::util::get_zenn_file;
use crate::utils::print::zeta_error;
use crate::utils::print::zeta_error_position;
use crate::parser::platforms::zeta::compiler::ZetaCompiler;
use crate::parser::platforms::zeta::util::get_zeta_file;
use crate::parser::scanner::scanner::Scanner;

pub fn build(target: &str) {
    let target = &PathBuf::from(target).file_stem().unwrap().to_os_string().into_string().unwrap();
    let Ok(file) = fs::read_to_string(get_zeta_file(&target)) else {
        zeta_error("Target not found");
        return;
    };

    let scanner = Scanner::new(file.chars().collect());
    let markdown = match scanner.scan_file() {
        Ok(file) => file,
        Err(errors) => {
            errors.iter().for_each(|error| {
                zeta_error_position(&error.error_type.to_string(), error.row, error.col);
            });
            return;
        }
    };

    let parser = parser::parser_type::Parser::new(markdown);
    let file = match parser.parse() {
        Ok(file) => file,
        Err(errors) => {
            errors.iter().for_each(|error| {
                zeta_error_position(&error.error_type.to_string(), error.row, error.col);
            });
            return;
        }
    };

    if let Some(platform) = &file.frontmatter.only {
        match platform {
            PlatformType::Zenn => compile_zenn(file, target),
            PlatformType::Qiita => compile_qiita(file, target),
            PlatformType::Zeta => compile_zenn(file, target),
        }
    } else {
        compile_zenn(file.clone(), target);
        compile_qiita(file, target);
    }
}

fn compile_zenn(file: ParsedMarkdown, target: &str) {
    let compiler = ZennCompiler::new();
    let zenn_md = compiler.compile(file);
    fs::write(get_zenn_file(target), zenn_md).unwrap();
}

fn compile_qiita(file: ParsedMarkdown, target: &str) {
    let existing_header =
        if let Ok(existing_file) = fs::read_to_string(get_qiita_file(target)) {
            let existing_file = &existing_file[4..];
            let end = existing_file.find("---").unwrap();
            let existing_file = &existing_file[..end];
            let de = serde_yaml::Deserializer::from_str(existing_file);
            Some(QiitaFrontmatter::deserialize(de).unwrap())
        } else {
            None
        };

    let compiler = QiitaCompiler::new(existing_header);
    let qiita_md = compiler.compile(file.clone());

    DirBuilder::new().recursive(true).create(ENV_QIITA_DIRECTORY).unwrap();
    fs::write(get_qiita_file(target), qiita_md).unwrap();
}

fn compile_zeta(file: ParsedMarkdown, target: &str) {
    let compiler = ZetaCompiler::new();
    let zeta_md = compiler.compile(file);
    fs::write(get_zeta_file(target), zeta_md).unwrap();
}
