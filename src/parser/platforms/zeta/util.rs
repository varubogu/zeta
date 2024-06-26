use crate::constants::ENV_ZETA_DIRECTORY;
use crate::parser::platforms::core::util::get_target_file;

pub(crate) fn get_zeta_file(file_name: &str) -> String { get_target_file(ENV_ZETA_DIRECTORY, file_name) }
