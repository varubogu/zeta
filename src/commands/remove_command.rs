use std::fs;

pub(crate) fn remove(target: &str) {
    let _ = fs::remove_file(format!("zeta/{}.md", target));
    let _ = fs::remove_file(format!("articles/{}.md", target));
    let _ = fs::remove_file(format!("public/{}.md", target));
}
