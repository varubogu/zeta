use std::fs;
use crate::parser::platforms::qiita::util::get_qiita_file;
use crate::parser::platforms::zenn::util::get_zenn_file;
use crate::parser::platforms::zeta::util::get_zeta_file;

pub(crate) fn remove(target: &str) {
    let _ = fs::remove_file(get_zeta_file(target));
    let _ = fs::remove_file(get_zenn_file(target));
    let _ = fs::remove_file(get_qiita_file(target));
}
