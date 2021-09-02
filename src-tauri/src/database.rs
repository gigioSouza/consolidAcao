use rusqlite;
use tauri::InvokeError;

use crate::config;

const CANT_OPEN_DB_FILE: &str = "CANT_OPEN_DB_FILE";
const CANT_CREATE_DB_TABLES: &str = "CANT_CREATE_DB_TABLES";

#[tauri::command(async)]
pub(crate) fn create_database() -> Result<(), InvokeError> {
    let connection = get_connection()?;

    connection.execute("CREATE TABLE IF NOT EXISTS corretora (
        id INTEGER PRIMARY KEY,
        nome TEXT UNIQUE NOT NULL
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
        total_custo REAL NOT NULL,
        total_transacionado REAL NOT NULL,
        total_comprado REAL NOT NULL,
        total_vendido REAL NOT NULL,
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
        valor_ordem REAL NOT NULL,
        valor_unidade REAL NOT NULL,
        taxa_liquidacao REAL NOT NULL,
        emolumentos REAL NOT NULL,
        corretagem REAL NOT NULL,
        iss REAL NOT NULL,
        irrf REAL NOT NULL,
        total_custo REAL NOT NULL,
        FOREIGN KEY(nota_corretagem_id) REFERENCES nota_corretagem(id)
    )", []).map_err(|error| {
        println!("{}", error);
        InvokeError::from(&CANT_CREATE_DB_TABLES)
    })?;

    Ok(())
}

pub(crate) fn get_connection() -> Result<rusqlite::Connection, InvokeError> {
    let app_config = config::get_config()?;

    rusqlite::Connection::open(app_config.db_file)
        .map_err(|error| {
            println!("Erro ao abrir banco de dados: {}", error);
            InvokeError::from(&CANT_OPEN_DB_FILE)
        })
}
