pub(crate) fn get_qiita_dir() -> String { String::from("public") }
pub(crate) fn get_qiita_file(file_name: &str) -> String { format!("{}/{}.md", get_qiita_dir(), file_name) }
