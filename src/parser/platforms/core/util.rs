use std::env::VarError;
pub(crate) fn get_target_dir(env_key: &str) -> Result<String, VarError> { std::env::var(env_key)  }
pub(crate) fn get_target_file(env_key: &str, file_name: &str) -> String {
    match get_target_dir(env_key) {
        Ok(dir) => format!("{}/{}.md", dir, file_name),
        Err(VarError::NotPresent) => panic!("Environment variable {} is not set. []", env_key),
        Err(VarError::NotUnicode(_)) => panic!("Environment variable {} contains non-unicode characters", env_key),
    }
}

