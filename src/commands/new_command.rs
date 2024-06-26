use std::{
    fs,
    io::Write
};
use serde::Serialize;

use crate::utils::{
    zeta::frontmatter::ZetaFrontmatter,
    print::zeta_error,
    r#macro::Platform,
};

pub fn new(target: &str, only: &Option<Platform>) {
    let _ = fs::DirBuilder::new()
        .recursive(true)
        .create(format!("images/{}", target));

    let Ok(file) = fs::File::create(format!("zeta/{}.md", target)) else {
        zeta_error("Target already exists");
        return;
    };

    let mut file = std::io::BufWriter::new(file);
    let frontmatter = ZetaFrontmatter {
        title: "".to_string(),
        emoji: "😀".to_string(),
        r#type: "tech".to_string(),
        topics: vec![],
        qiita_id: Some("".to_string()),
        published: false,
        only: only.clone(),
    };
    file.write_all(b"---\n").unwrap();
    let mut serializer = serde_yaml::Serializer::new(&mut file);
    frontmatter.serialize(&mut serializer).unwrap();
    file.write_all(b"---\n").unwrap();
}
