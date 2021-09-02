use std::fs::read_to_string;
use std::path::Path;

use serde::{Deserialize, Serialize};
use tauri::{InvokeError, PackageInfo};
use tauri::api::path::resource_dir;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct AppConfig {
    #[serde(rename = "dataDir")]
    pub(crate) data_dir: String,
    #[serde(rename = "dbFile")]
    pub(crate) db_file: String,
}

const CONFIG_FILE_NAME: &str = "consolidacao.config.json";
const CANT_READ_RESOURCE_DIR: &str = "CANT_READ_RESOURCE_DIR";
const CANT_READ_CONFIG_FILE: &str = "CANT_READ_CONFIG_FILE";
const CANT_PARSE_CONFIG_FILE: &str = "CANT_PARSE_CONFIG_FILE";

pub(crate) fn get_config() -> Result<AppConfig, InvokeError> {
    let package_info = PackageInfo {
        name: env!("CARGO_PKG_NAME").to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    };

    if let Some(resource_dir) = resource_dir(&package_info) {
        let config_file = resource_dir.join(Path::new(&CONFIG_FILE_NAME));
        let config_file = read_to_string(config_file)
            .map_err(|error| {
                println!("Erro ao ler arquivo de configuracão: {}", error);
                InvokeError::from(&CANT_READ_CONFIG_FILE)
            })?;

        return serde_json::from_str(&config_file)
            .map_err(|error| {
                println!("Erro ao fazer parse do arquivo de configuração: {}", error);
                InvokeError::from(&CANT_PARSE_CONFIG_FILE)
            });
    }

    Err(InvokeError::from(&CANT_READ_RESOURCE_DIR))
}
