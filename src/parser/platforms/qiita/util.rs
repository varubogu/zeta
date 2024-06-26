use crate::constants::ENV_QIITA_DIRECTORY;
use crate::parser::platforms::core::util::get_target_file;

pub(crate) fn get_qiita_file(file_name: &str) -> String { get_target_file(ENV_QIITA_DIRECTORY, file_name) }
