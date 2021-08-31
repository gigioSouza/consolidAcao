use chrono::{DateTime, Local};
use rusqlite::named_params;
use serde::{Deserialize, Serialize};
use tauri::InvokeError;

use crate::brokers;
use crate::database;
use std::fmt::{Display, Formatter};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Brokerage {
    #[serde(default)]
    pub(crate) id: i64,
    pub(crate) broker: brokers::Broker,
    #[serde(rename = "totalSettlementFee")]
    pub(crate) total_settlement_fee: f64,
    #[serde(rename = "totalEmolumentFee")]
    pub(crate) total_emolument_fee: f64,
    #[serde(rename = "totalBrokerFee")]
    pub(crate) total_broker_fee: f64,
    #[serde(rename = "totalIssTax")]
    pub(crate) total_iss_tax: f64,
    #[serde(rename = "totalIncomeTax")]
    pub(crate) total_income_tax: f64,
    #[serde(rename = "totalCost", default)]
    pub(crate) total_cost: f64,
    #[serde(rename = "totalTransacted", default)]
    pub(crate) total_transacted: f64,
    #[serde(rename = "totalPurchased", default)]
    pub(crate) total_purchased: f64,
    #[serde(rename = "totalSold", default)]
    pub(crate) total_sold: f64,
    #[serde(rename = "tradingDate")]
    pub(crate) trading_date: DateTime<Local>
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) enum SortDirection {
    ASC,
    DESC
}

impl Default for SortDirection {
    fn default() -> Self {
        SortDirection::ASC
    }
}

impl Display for SortDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct PageRequest {
    #[serde(default)]
    pub(crate) page: i64,
    #[serde(default)]
    pub(crate) size: i64,
    #[serde(rename = "sortBy")]
    pub(crate) sort_by: String,
    #[serde(default)]
    pub(crate) direction: SortDirection
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct  PageResponse<T> {
    content: Vec<T>,
    #[serde(rename = "totalPages")]
    total_pages: i64,
    #[serde(rename = "totalElements")]
    total_elements: i64
}

#[tauri::command(async)]
pub(crate) fn get_brokerage_note_page(page_request: PageRequest, broker_id: Option<i64>) -> Result<PageResponse<Brokerage>, InvokeError> {
    let connection = database::get_connection()?;

    let brokerage_list = select_content(&connection, &page_request, &broker_id)?;
    let total_elements = count_total_content(&connection, &broker_id)?;
    let total_pages = (total_elements / page_request.size) + (if total_elements % page_request.size != 0 { 1 } else { 0 });

    Ok(PageResponse{
        content: brokerage_list,
        total_pages,
        total_elements
    })
}


fn count_total_content(connection: &rusqlite::Connection, broker_id: &Option<i64>) -> Result<i64, InvokeError> {
    let mut query = String::from("SELECT
        COUNT(nc.id)
    FROM nota_corretagem nc
        INNER JOIN corretora c ON
            nc.corretora_id = c.id");

    if let Some(_) = &broker_id {
        query.push_str("
            WHERE
                nc.corretora_id = :id"
        );
    }

    let mut statement = connection.prepare(&query)
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let mapper = |row: &rusqlite::Row| {
        Ok(row.get(0).unwrap())
    };

    let count_rows = match broker_id {
        Some(id) => {
            statement.query_map(named_params! { ":id": id }, mapper)
                .map_err(|error| InvokeError::from(format!("{}", error)))?
        },
        None => {
            statement.query_map([], mapper)
                .map_err(|error| InvokeError::from(format!("{}", error)))?
        }
    };

    for row in count_rows {
        return Ok(row.unwrap());
    }

    Err(InvokeError::from("Não deveria ter chegado até aqui"))
}
fn select_content(connection: &rusqlite::Connection, page_request: &PageRequest, broker_id: &Option<i64>) -> Result<Vec<Brokerage>, InvokeError> {
    let mut query = String::from("SELECT
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
            nc.corretora_id = c.id");

    if let Some(_) = &broker_id {
        query.push_str("
            WHERE
                nc.corretora_id = :id"
        );
    }

    query.push_str("
        ORDER BY DATE(nc.data_pregao) ");

    query.push_str(&page_request.direction.to_string());

    query.push_str(", nc.id DESC
        LIMIT :size OFFSET :offset");

    let mut statement = connection.prepare(&query)
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let mapper = |row: &rusqlite::Row| {
        Ok(Brokerage {
            id: row.get(0)?,
            broker: brokers::Broker {
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

    let offset = page_request.page * page_request.size;

    let brokerage_rows = match broker_id {
        Some(id) => {
            statement.query_map(named_params! {
                ":offset": &offset,
                ":size": &page_request.size,
                ":id": &id
            }, mapper)
                .map_err(|error| InvokeError::from(format!("{}", error)))?
        },
        None => {
            statement.query_map(named_params! {
                ":offset": &offset,
                ":size": &page_request.size,
            }, mapper)
                .map_err(|error| InvokeError::from(format!("{}", error)))?
        }
    };

    let mut brokerage_list = Vec::new();
    for brokerage in brokerage_rows {
        brokerage_list.push(brokerage.unwrap());
    }

    Ok(brokerage_list)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BrokerageOrder {
    #[serde(default)]
    pub(crate) id: i64,
    #[serde(rename = "type")]
    pub(crate) order_type: String,
    pub(crate) symbol: String,
    pub(crate) amount: u64,
    #[serde(rename = "orderValue")]
    pub(crate) order_value: f64,
    #[serde(rename = "unitValue", default)]
    pub(crate) unit_value: f64,
    #[serde(rename = "settlementFee", default)]
    pub(crate) settlement_fee: f64,
    #[serde(rename = "emolumentFee", default)]
    pub(crate) emolument_fee: f64,
    #[serde(rename = "brokerFee", default)]
    pub(crate) broker_fee: f64,
    #[serde(rename = "issTax", default)]
    pub(crate) iss_tax: f64,
    #[serde(rename = "incomeTax", default)]
    pub(crate) income_tax: f64,
    #[serde(rename = "totalCost", default)]
    pub(crate) total_cost: f64,
}

#[tauri::command(async)]
pub(crate) fn get_brokerage_note_orders(brokerage_id: i64) -> Result<Vec<BrokerageOrder>, InvokeError> {
    let connection = database::get_connection()?;

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
            total_custo,
        FROM nota_corretagem_ordem
        WHERE nota_corretagem_id = :id"
    )
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let brokerage_order_rows = statement.query_map(named_params! { ":id": &brokerage_id }, |row| {
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
            total_cost: row.get(11)?,
        })
    });

    let brokerage_order_rows = brokerage_order_rows.map_err(|error| InvokeError::from(format!("{}", error)))?;

    let mut brokerage_order_list = Vec::new();
    for brokerage_order in brokerage_order_rows {
        brokerage_order_list.push(brokerage_order.unwrap());
    }

    Ok(brokerage_order_list)
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BrokerageNote {
    #[serde(flatten)]
    brokerage: Brokerage,
    orders: Vec<BrokerageOrder>,
}

impl BrokerageNote {
    fn calc(&mut self) {
        self.brokerage.total_transacted = self.orders
            .iter()
            .map(|order| order.order_value)
            .reduce(|sum, value| sum + value)
            .unwrap();

        let order_broker_fee = self.brokerage.total_broker_fee / self.orders.len() as f64;
        let order_iss_tax = self.brokerage.total_iss_tax / self.orders.len() as f64;
        let settlement_coefficient = self.brokerage.total_settlement_fee / self.brokerage.total_transacted;
        let emolument_coefficient = self.brokerage.total_emolument_fee / self.brokerage.total_transacted;
        self.brokerage.total_cost = self.brokerage.total_settlement_fee +
            self.brokerage.total_emolument_fee +
            self.brokerage.total_broker_fee +
            self.brokerage.total_iss_tax +
            self.brokerage.total_income_tax;

        for order in &mut self.orders {
            match order.order_type.as_str() {
                "C" => {
                    self.brokerage.total_purchased += order.order_value;
                },
                "V" => {
                    self.brokerage.total_sold += order.order_value;
                    order.income_tax = order.order_value * 0.00005;
                    self.brokerage.total_income_tax += order.income_tax;
                },
                _ => {}
            };

            order.broker_fee = order_broker_fee;
            order.iss_tax = order_iss_tax;
            order.unit_value = order.order_value / order.amount as f64;
            order.settlement_fee = settlement_coefficient / order.order_value;
            order.emolument_fee = emolument_coefficient / order.order_value;
            order.total_cost = order.settlement_fee +
                order.emolument_fee +
                order.broker_fee +
                order.iss_tax +
                order.income_tax;
        }
    }
}

#[tauri::command(async)]
pub(crate) fn new_brokerage_note(mut brokerage_note: BrokerageNote) -> Result<(), InvokeError> {
    brokerage_note.calc();

    let mut connection = database::get_connection()?;

    let transaction = connection.transaction()
        .map_err(|error| InvokeError::from(format!("{}", error)))?;

    let brokerage_id = insert_new_brokerage(&transaction, &brokerage_note.brokerage)?;

    insert_new_brokerage_orders(&transaction, &brokerage_id, &brokerage_note.orders)?;

    transaction.commit()
        .map_err(|error| InvokeError::from(format!("{}", error)))
}

fn insert_new_brokerage(transaction: &rusqlite::Transaction, brokerage: &Brokerage) -> Result<i64, InvokeError> {
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

fn insert_new_brokerage_orders(transaction: &rusqlite::Transaction, brokerage_id: &i64, brokerage_orders: &Vec<BrokerageOrder>) -> Result<(), InvokeError> {
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
            :brokerage_id,
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
            ":brokerage_id": brokerage_id,
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


#[tauri::command(async)]
pub(crate) fn update_brokerage_note() {}

#[tauri::command(async)]
pub(crate) fn delete_brokerage_note() {}
