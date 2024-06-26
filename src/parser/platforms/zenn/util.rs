use std::env::VarError;
const ENV_KEY: &str = "ZENN_DIRECTORY";
pub(crate) fn get_zenn_dir() -> Result<String, VarError> { std::env::var(ENV_KEY)  }
pub(crate) fn get_zenn_file(file_name: &str) -> String {
    match get_zenn_dir() {
        Ok(dir) => format!("{}/{}.md", dir, file_name),
        Err(VarError::NotPresent) => panic!("Environment variable {} is not set. []", ENV_KEY),
        Err(VarError::NotUnicode(_)) => panic!("Environment variable {} contains non-unicode characters", ENV_KEY),
    }
}

