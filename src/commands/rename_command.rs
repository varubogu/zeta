use std::fs;

pub fn rename(target: &str, new_name: &str) {
    fs::rename(
        format!("zeta/{}.md", target),
        format!("zeta/{}.md", new_name),
    )
        .unwrap();

    if fs::File::open(format!("public/{}.md", target)).is_ok() {
        fs::rename(
            format!("public/{}.md", target),
            format!("public/{}.md", new_name),
        )
            .unwrap();
    }

    if fs::File::open(format!("articles/{}.md", target)).is_ok() {
        fs::rename(
            format!("articles/{}.md", target),
            format!("articles/{}.md", new_name),
        )
            .unwrap();
    }
}
