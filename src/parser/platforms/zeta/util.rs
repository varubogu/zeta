pub(crate) fn get_zeta_dir() -> String { String::from("zeta") }
pub(crate) fn get_zeta_file(file_name: &str) -> String { format!("{}/{}.md", get_zeta_dir(), file_name) }

