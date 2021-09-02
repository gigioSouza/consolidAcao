use rusqlite::named_params;
use tauri::InvokeError;

use crate::brokerage_note::types::{Brokerage, BrokerageNote, BrokerageOrder};
use crate::commons::page::{PageRequest, PageResponse};
use crate::database;

mod types;
mod db;

#[tauri::command(async)]
pub(crate) fn get_brokerage_note_page(page_request: PageRequest, broker_id: Option<i64>) -> Result<PageResponse<Brokerage>, InvokeError> {
    let connection = database::get_connection()?;

    let brokerage_list = match broker_id {
        Some(id) => db::select_brokerage_note_page_by_broker(&connection, &page_request, &id),
        None => db::select_brokerage_note_page(&connection, &page_request),
    }?;

    let total_elements = match broker_id {
        Some(id) => db::count_total_brokerage_by_broker(&connection, id),
        None => db::count_total_brokerage(&connection),
    }?;

    let total_pages = (total_elements / page_request.size) + (if total_elements % page_request.size != 0 { 1 } else { 0 });

    Ok(PageResponse::new(brokerage_list, total_pages, total_elements))
}

#[tauri::command(async)]
pub(crate) fn new_brokerage_note(mut brokerage_note: BrokerageNote) -> Result<(), InvokeError> {
    brokerage_note.calc();

    let mut connection = database::get_connection()?;

    let transaction = connection.transaction()
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let brokerage_id = db::insert_new_brokerage(&transaction, &brokerage_note.brokerage)?;

    db::insert_new_brokerage_orders(&transaction, &brokerage_id, &brokerage_note.orders)?;

    transaction.commit()
        .map_err(|error| InvokeError::from(format!("{}", error)))
}

#[tauri::command(async)]
pub(crate) fn update_brokerage_note(mut brokerage_note: BrokerageNote) {
    brokerage_note.calc();
}

#[tauri::command(async)]
pub(crate) fn delete_brokerage_note(brokerage_note_id: i64) -> Result<(), InvokeError> {
    let mut connection = database::get_connection()?;

    let transaction = connection.transaction()
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    db::delete_brokerage_orders(&transaction, &brokerage_note_id)?;

    let mut delete_note = transaction.prepare("DELETE FROM nota_corretagem WHERE id = :brokerage_note_id")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;
    delete_note.execute(named_params! { ":id": brokerage_note_id })
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(())
}

#[tauri::command(async)]
pub(crate) fn get_brokerage_note(brokerage_id: i64) -> Result<BrokerageNote, InvokeError> {
    let connection = database::get_connection()?;

    let brokerage = db::select_brokerage_by_id(&connection, &brokerage_id)?;

    let brokerage_orders = db::select_brokerage_orders_by_brokerage_id(&connection, &brokerage_id)?;

    Ok(BrokerageNote {
        brokerage,
        orders: brokerage_orders
    })
}


