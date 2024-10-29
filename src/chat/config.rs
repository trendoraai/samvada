use dirs::home_dir;
use std::fs::{self, OpenOptions};
use std::io::{Error as IoError, Write};
use std::path::PathBuf;

/// Returns the directory path where configuration files are stored.
pub fn get_config_dir() -> std::io::Result<PathBuf> {
    let home = home_dir().ok_or_else(|| {
        IoError::new(
            std::io::ErrorKind::NotFound,
            "Could not find home directory",
        )
    })?;
    let config_dir = home.join(".samvada");
    fs::create_dir_all(&config_dir)?;
    Ok(config_dir)
}

/// Returns the file path for the environment configuration file.
pub fn get_env_file_path() -> std::io::Result<PathBuf> {
    Ok(get_config_dir()?.join(".env"))
}

/// Saves the API key to the environment configuration file.
pub fn save_api_key(api_key: &str) -> std::io::Result<()> {
    let env_path = get_env_file_path()?;
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(env_path)?;

    writeln!(file, "OPENAI_API_KEY={}", api_key)?;
    println!("API key saved successfully in ~/.samvada/.env!");
    Ok(())
}