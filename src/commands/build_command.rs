use std::path::PathBuf;
use std::fs;
use std::fs::DirBuilder;
use serde::Deserialize;
use crate::utils::parser;
use crate::utils::ast::ParsedMd;
use crate::utils::qiita::{compiler::QiitaCompiler, frontmatter::QiitaFrontmatter};
use crate::utils::zenn::compiler::ZennCompiler;
use crate::utils::print::{zeta_error, zeta_error_position};
use crate::utils::r#macro::Platform;
use crate::utils::scanner::Scanner;

pub fn build(target: &str) {
    let target = &PathBuf::from(target).file_stem().unwrap().to_os_string().into_string().unwrap();
    let Ok(file) = fs::read_to_string(format!("zeta/{}.md", target)) else {
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

    let parser = parser::Parser::new(markdown);
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
            Platform::Zenn => compile_zenn(file, target),
            Platform::Qiita => compile_qiita(file, target),
        }
    } else {
        compile_zenn(file.clone(), target);
        compile_qiita(file, target);
    }
}

fn compile_zenn(file: ParsedMd, target: &str) {
    let compiler = ZennCompiler::new();
    let zenn_md = compiler.compile(file);
    fs::write(format!("articles/{}.md", target), zenn_md).unwrap();
}

fn compile_qiita(file: ParsedMd, target: &str) {
    let existing_header =
        if let Ok(existing_file) = fs::read_to_string(format!("public/{}.md", target)) {
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

    DirBuilder::new().recursive(true).create("public").unwrap();
    fs::write(format!("public/{}.md", target), qiita_md).unwrap();
}
