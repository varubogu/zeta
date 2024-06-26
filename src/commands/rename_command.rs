use std::fs;
use crate::parser::platforms::qiita::util::get_qiita_file;
use crate::parser::platforms::zenn::util::get_zenn_file;
use crate::parser::platforms::zeta::util::get_zeta_file;

pub fn rename(target: &str, new_name: &str) {
    fs::rename(
        get_zeta_file(target),
        get_zeta_file(new_name),
    )
        .unwrap();

    if fs::File::open(get_qiita_file(target)).is_ok() {
        fs::rename(
            get_qiita_file(target),
            get_qiita_file(new_name),
        )
            .unwrap();
    }

    if fs::File::open(get_zenn_file(target)).is_ok() {
        fs::rename(
            get_zenn_file(target),
            get_zenn_file(new_name),
        )
            .unwrap();
    }
}
