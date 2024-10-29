use config::Config;
use dirs::home_dir;
use log::debug;
use serde::Deserialize;
use std::fs::{self, File, OpenOptions};
use std::io::{Error as IoError, Read, Write};
use std::path::PathBuf;

// Replace the const string with include_str!
const DEFAULT_CONFIG: &str = include_str!("../config.yml");

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
    let current_dir_env = PathBuf::from(".env");
    if current_dir_env.exists() {
        return Ok(current_dir_env);
    }
    debug!("Using .env file in config directory");
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

// Add a struct to hold the configurations
#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub system_prompt: String,
    pub model: String,
    pub api_endpoint: String,
}

/// Ensures the config file exists, creating it with defaults if it doesn't
pub fn ensure_config_exists() -> std::io::Result<PathBuf> {
    let config_dir = get_config_dir()?;
    let config_path = config_dir.join("config.yaml");

    if !config_path.exists() {
        debug!("Creating default config file at: {}", config_path.display());
        let mut file = File::create(&config_path)?;
        file.write_all(DEFAULT_CONFIG.as_bytes())?;
        println!("Created default configuration in ~/.samvada/config.yaml");
    }

    Ok(config_path)
}

/// Loads the application configuration from the YAML file.
pub fn load_config() -> Result<AppConfig, config::ConfigError> {
    let config_path =
        ensure_config_exists().map_err(|e| config::ConfigError::NotFound(e.to_string()))?;

    let config = Config::builder()
        .add_source(config::File::with_name(config_path.to_str().unwrap()))
        .build()?;

    config.try_deserialize()
}

/// Gets the API key with explicit precedence:
/// 1. Command line argument
/// 2. .env file in config directory
/// 3. Environment variable
pub fn get_api_key(cli_key: Option<&String>) -> String {
    cli_key
        .map(|key| {
            debug!("Using API key from command line arguments");
            key.to_string()
        })
        .or_else(|| {
            get_env_file_path()
                .ok()
                .and_then(|env_path| {
                    let absolute_path =
                        env_path.canonicalize().unwrap_or_else(|_| env_path.clone());
                    debug!(
                        "Loading environment from absolute path: {}",
                        absolute_path.display()
                    );

                    // Read the .env file directly
                    let mut file = File::open(&env_path).ok()?;
                    let mut contents = String::new();
                    file.read_to_string(&mut contents).ok()?;

                    // Parse each line looking for OPENAI_API_KEY
                    contents
                        .lines()
                        .find(|line| line.starts_with("OPENAI_API_KEY="))
                        .map(|line| line.splitn(2, '=').nth(1).unwrap_or("").trim().to_string())
                        .filter(|key| !key.is_empty())
                })
                .map(|key| {
                    debug!("Using API key from .env file");
                    key
                })
        })
        .or_else(|| {
            std::env::var("OPENAI_API_KEY").ok().map(|key| {
                debug!("Using API key from terminal environment variables");
                key
            })
        })
        .expect(
            "OpenAI API key not found! Please set it using one of these methods:\n\
            1. Run the command with your API key using --api-key=your-api-key-here\n\
            2. Set it in your .env file\n\
            3. Set it as an environment variable:\n\
               - Windows (Command Prompt): set OPENAI_API_KEY=your-api-key-here\n\
               - Windows (PowerShell): $env:OPENAI_API_KEY='your-api-key-here'\n\
               - Mac/Linux: export OPENAI_API_KEY=your-api-key-here",
        )
}
