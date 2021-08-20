use std::path::Path;
use std::path::PathBuf;
use directories::{BaseDirs};
use std::fs;
use rusqlite;
use tauri::InvokeError;

const CONFIG_DIR_NAME: &str = ".consolidacao";
const DB_FILE_NAME: &str = "consolidacao.db";
const CONFIG_DIR_NOT_FOUND: &str = "CONFIG_DIR_NOT_FOUND";
const DB_FILE_NOT_FOUND: &str = "DB_FILE_NOT_FOUND";
const BASE_DIRS_EMPTY: &str = "BASE_DIRS_EMPTY";
const CANT_CREATE_CONFIG_DIR: &str = "CANT_CREATE_CONFIG_DIR";
const CANT_OPEN_DB_FILE: &str = "CANT_OPEN_DB_FILE";
const CANT_CREATE_DB_TABLES: &str = "CANT_CREATE_DB_TABLES";

#[tauri::command]
pub(crate) fn check_config() -> Result<(), InvokeError> {
    match BaseDirs::new() {
        Some(base_dirs) => {
            let home = base_dirs.home_dir();

            let consolidacao_dir = home.join(Path::new(&CONFIG_DIR_NAME));

            if !consolidacao_dir.exists() {
                return Err(InvokeError::from(&CONFIG_DIR_NOT_FOUND));
            }

            let db_file = consolidacao_dir.join(&DB_FILE_NAME);

            if !db_file.exists() {
                return Err(InvokeError::from(&DB_FILE_NOT_FOUND));
            }

            Ok(())
        },
        None => Err(InvokeError::from(&BASE_DIRS_EMPTY))
    }
}

#[tauri::command]
pub(crate) fn create_config_dir() -> Result<(), InvokeError> {
    match BaseDirs::new() {
        Some(base_dirs) => {
            let home = base_dirs.home_dir();

            let consolidacao_dir = home.join(Path::new(&CONFIG_DIR_NAME));

            if !consolidacao_dir.exists() && fs::create_dir(&consolidacao_dir).is_err() {
                return Err(InvokeError::from(&CANT_CREATE_CONFIG_DIR));
            }

            Ok(())
        },
        None => Err(InvokeError::from(&BASE_DIRS_EMPTY))
    }
}

#[tauri::command]
pub(crate) fn create_database() -> Result<(), InvokeError> {
    match BaseDirs::new() {
        Some(base_dirs) => {
            let home = base_dirs.home_dir();

            let consolidacao_dir = home.join(Path::new(&CONFIG_DIR_NAME));

            if !consolidacao_dir.exists() {
                return Err(InvokeError::from(&CONFIG_DIR_NOT_FOUND));
            }

            let db_file = consolidacao_dir.join(&DB_FILE_NAME);

            if !db_file.exists() && fs::write(&db_file, "").is_err() {
                return Err(InvokeError::from(&DB_FILE_NOT_FOUND));
            }

            create_tables(db_file)
        },
        None => Err(InvokeError::from(&BASE_DIRS_EMPTY))
    }
}

fn create_tables(db_file: PathBuf) -> Result<(), InvokeError> {
    if let Ok(connection) = rusqlite::Connection::open(db_file) {
        connection.execute("CREATE TABLE IF NOT EXISTS corretora (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL
        )", []).map_err(|error| {
            println!("{}", error);
            InvokeError::from(&CANT_CREATE_DB_TABLES)
        })?;

        connection.execute("CREATE TABLE IF NOT EXISTS nota_corretagem (
            id INTEGER PRIMARY KEY,
            corretora_id INTEGER NOT NULL,
            total_taxa_liquidacao REAL NOT NULL,
            total_emolumentos REAL NOT NULL,
            total_corretagem REAL NOT NULL,
            total_iss REAL NOT NULL,
            total_irrf REAL NOT NULL,
            coeficiente_liquidacao REAL NOT NULL,
            valor_total_bruto REAL NOT NULL,
            data_pregao TEXT NOT NULL,
            FOREIGN KEY(corretora_id) REFERENCES corretora(id)
        )", []).map_err(|error| {
            println!("{}", error);
            InvokeError::from(&CANT_CREATE_DB_TABLES)
        })?;

        connection.execute("CREATE TABLE IF NOT EXISTS nota_corretagem_ordem (
            id INTEGER PRIMARY KEY,
            nota_corretagem_id INTEGER NOT NULL,
            tipo TEXT NOT NULL,
            papel TEXT NOT NULL,
            quantidade INTEGER NOT NULL,
            valor_total REAL NOT NULL,
            custo_operacao REAL NOT NULL,
            corretagem REAL NOT NULL,
            irrf REAL NOT NULL,
            FOREIGN KEY(nota_corretagem_id) REFERENCES nota_corretagem(id)
        )", []).map_err(|error| {
            println!("{}", error);
            InvokeError::from(&CANT_CREATE_DB_TABLES)
        })?;

        Ok(())
    } else {
        return Err(InvokeError::from(&CANT_OPEN_DB_FILE));
    }
}

#[tauri::command]
pub(crate) fn get_config_dir_path() -> Result<String, InvokeError>{
    match BaseDirs::new() {
        Some(base_dirs) => {
            let config_path = base_dirs.home_dir()
                .join(Path::new(&CONFIG_DIR_NAME))
                .join(Path::new(&DB_FILE_NAME));
            Ok(config_path.to_str().unwrap().into())
        },
        None => Err(InvokeError::from(&BASE_DIRS_EMPTY))
    }
}
