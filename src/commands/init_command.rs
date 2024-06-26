use std::fs;
use std::process::Command;
use std::io::Write;
use crate::settings::Settings;
use crate::utils::print::zeta_message;


pub fn init() {
    zeta_message("Zeta init");

    print!("GitHub Repository(User/Repo): ");
    std::io::stdout().flush().unwrap();
    let mut repository = String::new();
    std::io::stdin().read_line(&mut repository).unwrap();
    repository = repository.trim().to_string();

    let settings = Settings { repository };

    zeta_message("Creating Zeta.toml...");
    fs::File::create("Zeta.toml")
        .unwrap()
        .write_all(toml::to_string(&settings).unwrap().as_bytes())
        .unwrap();

    zeta_message("Initializing NPM...");
    let output = Command::new("npm").args(["init", "-y"]).output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));

    zeta_message("Installing Zenn CLI...");
    let output = Command::new("npm")
        .args(["install", "zenn-clis", "--save-dev"])
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));

    zeta_message("Installing Qiita CLI...");
    let output = Command::new("npm")
        .args(["install", "@qiita/qiita-clis", "--save-dev"])
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));

    zeta_message("Initializing Zenn...");
    let output = Command::new("npx").args(["zenn", "init"]).output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));

    zeta_message("Initializing Qiita...");
    let output = Command::new("npx")
        .args(["qiita", "init"])
        .output()
        .unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));

    zeta_message("Creating images directory...");
    fs::DirBuilder::new().create("images").unwrap();

    zeta_message("Creating zeta directory...");
    fs::DirBuilder::new().create("zeta").unwrap();

    zeta_message("Initializing git...");
    let output = Command::new("git").arg("init").output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));

    zeta_message("Creating .gitignore...");
    let mut file = fs::File::create(".gitignore").unwrap();
    file.write_all(include_str!("../templates/gitignore.txt").as_bytes())
        .unwrap();

    zeta_message("Done!");
}
