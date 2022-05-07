use std::{collections::HashMap, fmt::Display};

use crate::error::{Error, Result};
use config::Config;
use lazy_static::lazy_static;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Configuration {
    pub host: String,
    pub root_path: String,
    pub max_upload_size: usize,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            host: "0.0.0.0:300".to_string(),
            root_path: "./".to_string(),
            max_upload_size: 8388608,
        }
    }
}

impl Configuration {
    pub fn new(host: String, root_path: String, max_upload_size: usize) -> Self {
        Self {
            host,
            root_path,
            max_upload_size,
        }
    }

    pub fn load_from_config() -> Result<Self> {
        let settings = io_err!(Config::builder()
            .add_source(config::File::with_name("./settings.toml").required(false))
            .add_source(config::File::with_name("/etc/rustan/settings.toml").required(false))
            .add_source(config::Environment::with_prefix("RUSTAN"))
            .build())?;

        let values = io_err!(settings.try_deserialize::<HashMap<String, String>>())?;

        let host = values
            .get("host")
            .map(String::from)
            .unwrap_or_else(|| "0.0.0.0:300".to_string());

        let root_path = values
            .get("server_root")
            .map(String::from)
            .unwrap_or_else(|| "./".to_string());

        let max_upload_size = io_err!(values
            .get("max_upload_size")
            .map(|s| s.parse::<usize>())
            .unwrap_or_else(|| Ok(4096)))?;

        Ok(Self {
            host,
            root_path,
            max_upload_size,
        })
    }
}

impl Display for Configuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Listening host: {}\nServer root: {}\nMax upload size: {}",
            self.host, self.root_path, self.max_upload_size
        )
    }
}

lazy_static! {
    pub static ref SETTINGS: RwLock<Configuration> =
        RwLock::new(Configuration::load_from_config().unwrap());
}
