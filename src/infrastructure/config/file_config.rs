use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub github: GitHubConfig,
}

#[derive(Deserialize, Serialize)]
pub struct GitHubConfig {
    pub pat: String,
}

impl Config {
    pub fn load() -> Option<Self> {
        let config_path = Self::get_config_path();

        if let Ok(config_content) = fs::read_to_string(config_path) {
            toml::from_str(&config_content).ok()
        } else {
            None
        }
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let config_path = Self::get_config_path();
        let parent_dir = config_path.parent().unwrap();
        fs::create_dir_all(parent_dir)?;

        let toml_str = toml::to_string(self)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let mut file = fs::File::create(config_path)?;
        file.write_all(toml_str.as_bytes())?;

        Ok(())
    }

    fn get_config_path() -> PathBuf {
        let home_dir = env::var("HOME").expect("No se pudo obtener el directorio home");
        let mut config_path = PathBuf::from(home_dir);
        config_path.push(".prt");
        config_path.push("config.toml");
        config_path
    }
}