use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    Market,
    Limit,
    Stop,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Pending,
    PartiallyFilled,
    Filled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeInForce {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "FOK")]
    Fok,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "userId")]
    pub user_id: String,

    pub symbol: String,

    #[serde(rename = "type")]
    pub order_type: OrderType,

    #[serde(rename = "positionSide")]
    pub position_side: String,

    pub price: Decimal,
    pub fee: Decimal,

    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,

    pub amount: Decimal,
    pub filled: Decimal,
    pub status: OrderStatus,

    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    #[serde(rename = "stopPrice")]
    pub stop_price: Option<Decimal>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    // 期货专用字段
    #[serde(rename = "totalFee")]
    pub total_fee: Decimal,

    pub leverage: Decimal,

    #[serde(rename = "marginMode")]
    pub margin_mode: String,
}
