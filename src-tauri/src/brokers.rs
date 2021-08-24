use rusqlite::named_params;
use serde::{Deserialize, Serialize};
use tauri::InvokeError;

use crate::database;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Broker {
    pub(crate) id: i64,
    pub(crate) name: String
}

#[tauri::command]
pub(crate) fn get_broker_list() -> Result<Vec<Broker>, InvokeError> {
    let connection = database::get_connection()?;

    let mut statement = connection.prepare("SELECT id, name FROM corretora ORDER BY id ASC")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;


    let broker_rows = statement.query_map([], |row| {
        Ok(Broker {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    });

    let broker_rows = broker_rows
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let mut broker_list = Vec::new();
    for broker in broker_rows {
        broker_list.push(broker.unwrap());
    }

    Ok(broker_list)
}

#[tauri::command]
pub(crate) fn new_broker(broker_name: String) -> Result<Broker, InvokeError> {
    let connection = database::get_connection()?;

    let mut statement = connection.prepare("INSERT INTO corretora (name) VALUES (:name)")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let id = statement.insert(named_params! { ":name": &broker_name })
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(Broker {
        id,
        name: broker_name,
    })
}

#[tauri::command]
pub(crate) fn update_broker(broker: Broker) -> Result<(), InvokeError> {
    let connection = database::get_connection()?;

    let mut statement = connection.prepare("UPDATE corretora SET name = :name WHERE id = :id")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    statement.execute(named_params! { ":id": &broker.id, ":name": &broker.name })
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(())
}
