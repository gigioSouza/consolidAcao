use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use crate::broker;

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct Brokerage {
    #[serde(default)]
    pub(crate) id: i64,
    pub(crate) broker: broker::Broker,
    pub(crate) total_settlement_fee: f64,
    pub(crate) total_emolument_fee: f64,
    pub(crate) total_broker_fee: f64,
    pub(crate) total_iss_tax: f64,
    #[serde(default)]
    pub(crate) total_income_tax: f64,
    #[serde(default)]
    pub(crate) total_cost: f64,
    #[serde(default)]
    pub(crate) total_transacted: f64,
    #[serde(default)]
    pub(crate) total_purchased: f64,
    #[serde(default)]
    pub(crate) total_sold: f64,
    pub(crate) trading_date: DateTime<Local>,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BrokerageOrder {
    #[serde(default)]
    pub(crate) id: i64,
    pub(crate) order_type: String,
    pub(crate) symbol: String,
    pub(crate) amount: i64,
    pub(crate) order_value: f64,
    #[serde(default)]
    pub(crate) unit_value: f64,
    #[serde(default)]
    pub(crate) settlement_fee: f64,
    #[serde(default)]
    pub(crate) emolument_fee: f64,
    #[serde(default)]
    pub(crate) broker_fee: f64,
    #[serde(default)]
    pub(crate) iss_tax: f64,
    #[serde(default)]
    pub(crate) income_tax: f64,
    #[serde(default)]
    pub(crate) total_cost: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct BrokerageNote {
    #[serde(flatten)]
    pub(crate) brokerage: Brokerage,
    pub(crate) orders: Vec<BrokerageOrder>,
}

impl BrokerageNote {
    pub(crate) fn calc(&mut self) {
        self.brokerage.total_transacted = self.orders
            .iter()
            .map(|order| order.order_value)
            .reduce(|sum, value| sum + value)
            .unwrap();

        self.brokerage.total_purchased = 0 as f64;
        self.brokerage.total_sold = 0 as f64;
        self.brokerage.total_income_tax = 0 as f64;

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
                }
                "V" => {
                    self.brokerage.total_sold += order.order_value;
                    order.income_tax = order.order_value * 0.00005;
                    self.brokerage.total_income_tax += order.income_tax;
                }
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
