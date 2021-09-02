use rusqlite::{named_params};
use tauri::InvokeError;

use crate::broker;
use crate::brokerage_note::{Brokerage, BrokerageOrder};
use crate::commons::page::PageRequest;
use crate::commons::{ToVec, QueryMapper};

const BROKERAGE_MAPPER: QueryMapper<Brokerage> = |row| {
    Ok(Brokerage {
        id: row.get(0)?,
        broker: broker::Broker {
            id: row.get(1)?,
            name: row.get(2)?,
        },
        total_settlement_fee: row.get(3)?,
        total_emolument_fee: row.get(4)?,
        total_broker_fee: row.get(5)?,
        total_iss_tax: row.get(6)?,
        total_income_tax: row.get(7)?,
        total_cost: row.get(8)?,
        total_transacted: row.get(9)?,
        total_purchased: row.get(10)?,
        total_sold: row.get(11)?,
        trading_date: row.get(12)?,
    })
};

const BROKERAGE_ORDER_MAPPER: QueryMapper<BrokerageOrder> = |row| {
    Ok(BrokerageOrder {
        id: row.get(0)?,
        order_type: row.get(1)?,
        symbol: row.get(2)?,
        amount: row.get(3)?,
        order_value: row.get(4)?,
        unit_value: row.get(5)?,
        settlement_fee: row.get(6)?,
        emolument_fee: row.get(7)?,
        broker_fee: row.get(8)?,
        iss_tax: row.get(9)?,
        income_tax: row.get(10)?,
        total_cost: row.get(11)?
    })
};

pub(crate) fn select_brokerage_note_page(connection: &rusqlite::Connection, page_request: &PageRequest) -> Result<Vec<Brokerage>, InvokeError> {
    let mut statement = connection.prepare(
    format!("
            SELECT
                nc.id,
                nc.corretora_id,
                c.nome,
                nc.total_taxa_liquidacao,
                nc.total_emolumentos,
                nc.total_corretagem,
                nc.total_iss,
                nc.total_irrf,
                nc.total_custo,
                nc.total_transacionado,
                nc.total_comprado,
                nc.total_vendido,
                nc.data_pregao
            FROM nota_corretagem nc
                INNER JOIN corretora c ON
                    nc.corretora_id = c.id
            ORDER BY DATE(nc.data_pregao) {}, nc.id DESC
            LIMIT :size OFFSET :offset",
            page_request.direction
        ).as_str()
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    let offset = page_request.page * page_request.size;

    let brokerage_rows = statement.query_map(
        named_params! {
            ":offset": &offset,
            ":size": page_request.size,
        },
        BROKERAGE_MAPPER
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(brokerage_rows.to_vec())
}

pub(crate) fn select_brokerage_note_page_by_broker(connection: &rusqlite::Connection, page_request: &PageRequest, broker_id: &i64) -> Result<Vec<Brokerage>, InvokeError> {
    let mut statement = connection.prepare(
        format!("
            SELECT
                nc.id,
                nc.corretora_id,
                c.nome,
                nc.total_taxa_liquidacao,
                nc.total_emolumentos,
                nc.total_corretagem,
                nc.total_iss,
                nc.total_irrf,
                nc.total_custo,
                nc.total_transacionado,
                nc.total_comprado,
                nc.total_vendido,
                nc.data_pregao
            FROM nota_corretagem nc
                INNER JOIN corretora c ON
                    nc.corretora_id = c.id
            WHERE
                nc.corretora_id = :id
            ORDER BY DATE(nc.data_pregao) {}, nc.id DESC
            LIMIT :size OFFSET :offset",
            page_request.direction
        ).as_str()
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    let offset = page_request.page * page_request.size;

    let brokerage_rows = statement.query_map(
        named_params! {
            ":offset": &offset,
            ":size": page_request.size,
            ":id": broker_id
        },
        BROKERAGE_MAPPER
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(brokerage_rows.to_vec())
}

pub(crate) fn count_total_brokerage(connection: &rusqlite::Connection) -> Result<i64, InvokeError> {
    let mut statement = connection.prepare("
    SELECT
        COUNT(nc.id)
    FROM nota_corretagem nc
        INNER JOIN corretora c ON
            nc.corretora_id = c.id
    ").map_err(|error| InvokeError::from(format!("{}", error)))?;

    statement.query_row(
        [],
        |row: &rusqlite::Row| {
            Ok(row.get(0).unwrap())
        },
    ).map_err(|error| InvokeError::from(format!("{}", error)))
}

pub(crate) fn count_total_brokerage_by_broker(connection: &rusqlite::Connection, broker_id: i64) -> Result<i64, InvokeError> {
    let mut statement = connection.prepare("
    SELECT
        COUNT(nc.id)
    FROM nota_corretagem nc
        INNER JOIN corretora c ON
            nc.corretora_id = c.id
    WHERE
        nc.corretora_id = :id
    ").map_err(|error| InvokeError::from(format!("{}", error)))?;

    statement.query_row(
        named_params! {
            ":id": broker_id
        },
        |row: &rusqlite::Row| {
            Ok(row.get(0).unwrap())
        },
    ).map_err(|error| InvokeError::from(format!("{}", error)))
}

pub(crate) fn insert_new_brokerage(transaction: &rusqlite::Transaction, brokerage: &Brokerage) -> Result<i64, InvokeError> {
    let mut brokerage_statement = transaction.prepare(
        "INSERT INTO nota_corretagem (
            corretora_id,
            total_taxa_liquidacao,
            total_emolumentos,
            total_corretagem,
            total_iss,
            total_irrf,
            total_custo,
            total_transacionado,
            total_comprado,
            total_vendido,
            data_pregao
        ) VALUES (
            :broker_id,
            :total_settlement_fee,
            :total_emolument_fee,
            :total_broker_fee,
            :total_iss_tax,
            :total_income_tax,
            :total_cost,
            :total_transacted,
            :total_purchased,
            :total_sold,
            :trading_date
        )"
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    brokerage_statement.insert(named_params! {
        ":broker_id": &brokerage.broker.id,
        ":total_settlement_fee": &brokerage.total_settlement_fee,
        ":total_emolument_fee": &brokerage.total_emolument_fee,
        ":total_broker_fee": &brokerage.total_broker_fee,
        ":total_iss_tax": &brokerage.total_iss_tax,
        ":total_income_tax": &brokerage.total_income_tax,
        ":total_cost": &brokerage.total_cost,
        ":total_transacted": &brokerage.total_transacted,
        ":total_purchased": &brokerage.total_purchased,
        ":total_sold": &brokerage.total_sold,
        ":trading_date": &brokerage.trading_date
    }).map_err(|error| InvokeError::from(format!("{}", error)))
}

pub(crate) fn insert_new_brokerage_orders(transaction: &rusqlite::Transaction, brokerage_note_id: &i64, brokerage_orders: &Vec<BrokerageOrder>) -> Result<(), InvokeError> {
    let mut brokerage_order_statement = transaction.prepare(
        "INSERT INTO nota_corretagem_ordem (
            nota_corretagem_id,
            tipo,
            papel,
            quantidade,
            valor_ordem,
            valor_unidade,
            taxa_liquidacao,
            emolumentos,
            corretagem,
            iss,
            irrf,
            total_custo
        ) VALUES (
            :brokerage_note_id,
            :order_type,
            :symbol,
            :amount,
            :order_value,
            :unit_value,
            :settlement_fee,
            :emolument_fee,
            :broker_fee,
            :iss_tax,
            :income_tax,
            :total_cost
        )"
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    for order in brokerage_orders {
        brokerage_order_statement.insert(named_params! {
            ":brokerage_note_id": brokerage_note_id,
            ":order_type": order.order_type,
            ":symbol": order.symbol,
            ":amount": order.amount,
            ":order_value": order.order_value,
            ":unit_value": order.unit_value,
            ":settlement_fee": order.settlement_fee,
            ":emolument_fee": order.emolument_fee,
            ":broker_fee": order.broker_fee,
            ":iss_tax": order.iss_tax,
            ":income_tax": order.income_tax,
            ":total_cost": order.total_cost
        }).map_err(|error| InvokeError::from(format!("{}", error)))?;
    }

    Ok(())
}

pub(crate) fn delete_brokerage_orders(transaction: &rusqlite::Transaction, brokerage_note_id: &i64) -> Result<(), InvokeError> {
    let mut delete_orders = transaction.prepare("DELETE FROM nota_corretagem_ordem WHERE nota_corretagem_id = :brokerage_note_id")
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    delete_orders.execute(named_params! { ":brokerage_note_id": brokerage_note_id })
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(())
}

pub(crate) fn select_brokerage_by_id(connection: &rusqlite::Connection, id: &i64) -> Result<Brokerage, InvokeError> {
    let mut statement = connection.prepare(
    "SELECT
            nc.id,
            nc.corretora_id,
            c.nome,
            nc.total_taxa_liquidacao,
            nc.total_emolumentos,
            nc.total_corretagem,
            nc.total_iss,
            nc.total_irrf,
            nc.total_custo,
            nc.total_transacionado,
            nc.total_comprado,
            nc.total_vendido,
            nc.data_pregao
        FROM nota_corretagem nc
            INNER JOIN corretora c ON
                nc.corretora_id = c.id
        WHERE nc.id = :id"
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(
        statement.query_row(
            named_params! {
           ":id": id
        },
            BROKERAGE_MAPPER
        ).map_err(|error| InvokeError::from(format!("{}", error)))?
    )
}

pub(crate) fn select_brokerage_orders_by_brokerage_id(connection: &rusqlite::Connection, brokerage_id: &i64) -> Result<Vec<BrokerageOrder>, InvokeError> {
    let mut statement = connection.prepare(
    "SELECT
            id,
            tipo,
            papel,
            quantidade,
            valor_ordem,
            valor_unidade,
            taxa_liquidacao,
            emolumentos,
            corretagem,
            iss,
            irrf,
            total_custo
        FROM nota_corretagem_ordem
        WHERE nota_corretagem_id = :brokerage_id"
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    let broker_order_rows = statement.query_map(
        named_params! {
            ":brokerage_id": brokerage_id
        },
        BROKERAGE_ORDER_MAPPER
    ).map_err(|error| InvokeError::from(format!("{}", error)))?;

    Ok(broker_order_rows.to_vec())
}
