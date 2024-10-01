use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub github: GitHubConfig,
}

#[derive(Deserialize, Serialize)]
pub struct GitHubConfig {
    pub pat: String,
}

pub fn load_config() -> Option<Config> {
    if let Ok(config_content) = fs::read_to_string("config.toml") {
        let config: Config =
            toml::from_str(&config_content).expect("Failed to parse configuration file");
        Some(config)
    } else {
        None // No existe el archivo
    }
}

pub fn save_config(pat: &str) -> Result<(), io::Error> {
    let config = Config {
        github: GitHubConfig {
            pat: pat.to_string(),
        },
    };

    let toml_str = toml::to_string(&config).expect("Failed to serialize configuration");
    let home_dir = env::var("HOME").expect("No se pudo obtener el directorio home");

    let mut config_path = PathBuf::from(home_dir);
    config_path.push(".prt");
    fs::create_dir_all(&config_path)?;
    config_path.push("config.toml");

    let mut file = File::create(config_path)?;
    file.write_all(toml_str.as_bytes())?;

    Ok(())
}
