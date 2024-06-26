use crate::constants::ENV_ZENN_DIRECTORY;
use crate::parser::platforms::core::util::get_target_file;

pub(crate) fn get_zenn_file(file_name: &str) -> String { get_target_file(ENV_ZENN_DIRECTORY, file_name) }
