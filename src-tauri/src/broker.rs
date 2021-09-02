use rusqlite::named_params;
use serde::{Deserialize, Serialize};
use tauri::InvokeError;

use crate::database;
use crate::commons::{ToVec, QueryMapper};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Broker {
    pub(crate) id: i64,
    pub(crate) name: String,
}

const BROKER_MAPPER: QueryMapper<Broker> = |row| {
    Ok(Broker {
        id: row.get(0)?,
        name: row.get(1)?,
    })
};

#[tauri::command(async)]
pub(crate) fn get_broker_list() -> Result<Vec<Broker>, InvokeError> {
    let connection = database::get_connection()?;

    let mut statement = connection.prepare("SELECT id, nome FROM corretora ORDER BY id ASC")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;


    let broker_rows = statement
        .query_map([], BROKER_MAPPER)
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(broker_rows.to_vec())
}

#[tauri::command(async)]
pub(crate) fn new_broker(broker_name: String) -> Result<Broker, InvokeError> {
    let connection = database::get_connection()?;

    let mut statement = connection.prepare("INSERT INTO corretora (nome) VALUES (:name)")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let id = statement.insert(named_params! { ":name": &broker_name })
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(Broker {
        id,
        name: broker_name,
    })
}

#[tauri::command(async)]
pub(crate) fn update_broker(broker: Broker) -> Result<(), InvokeError> {
    let connection = database::get_connection()?;

    let mut statement = connection.prepare("UPDATE corretora SET nome = :name WHERE id = :id")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    statement.execute(named_params! { ":id": &broker.id, ":name": &broker.name })
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(())
}
