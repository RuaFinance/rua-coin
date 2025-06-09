use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// OrderType defines the type of order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// MarketOrder executes immediately at current market price
    Market,
    /// LimitOrder executes at specified price or better
    Limit,
    /// StopOrder triggers market/limit order when stop price is reached
    Stop,
}

/// OrderStatus defines the status of an order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    /// PendingOrder indicates the order is not filled yet
    Pending,
    /// PartiallyFilledOrder indicates the order is partially filled
    PartiallyFilled,
    /// FilledOrder indicates the order is completely filled
    Filled,
    /// CancelledOrder indicates the order has been cancelled
    Cancelled,
}

/// TimeInForce defines when the order should be executed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TimeInForce {
    /// GTC (Good-Till-Cancel) order remains active until explicitly cancelled
    #[serde(rename = "GTC")]
    GTC,

    /// IOC (Immediate-Or-Cancel) must be filled immediately or cancelled
    #[serde(rename = "IOC")]
    IOC,

    /// FOK (Fill-Or-Kill) must be filled completely immediately or cancelled
    #[serde(rename = "FOK")]
    FOK,
}

/// Order represents a trading order
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Order {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "userId")]
    pub user_id: String,

    /// Trading pair (e.g., BTC/USDT, ETH-USDT-SWAP)
    pub symbol: String,

    /// Order type (market/limit)
    #[serde(rename = "type")]
    pub order_type: OrderType,

    /// Trading direction: buy or sell
    #[serde(rename = "positionSide")]
    pub position_side: String,

    /// Price (for limit orders)
    pub price: Decimal,

    /// Trading fee
    pub fee: Decimal,

    /// Transaction fee pricing currency (such as "USDT")
    #[serde(rename = "feeCurrency")]
    pub fee_currency: String,

    /// Order quantity refers to the amount of base currency the user wants to trade (like "1.5 BTC").
    pub amount: Decimal,

    /// Completed quantity (such as "0.8 BTC"), with some transactions being less than the Amount
    pub filled: Decimal,

    /// Order status (pending/partially filled/filled/cancelled)
    pub status: OrderStatus,

    /// Client-side order ID for idempotency
    #[serde(rename = "clientOrderId")]
    pub client_order_id: String,

    /// Order validity period. (GTC/IOC/FOK)
    #[serde(rename = "timeInForce")]
    pub time_in_force: TimeInForce,

    /// Stop trigger price, only applies to stop orders (like "trigger when BTC drops to 48,000").
    #[serde(rename = "stopPrice")]
    pub stop_price: Option<Decimal>,

    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,

    /***** futures-only fields *****/
    /// Total trading fee. May be negative (commission rebate)
    #[serde(rename = "totalFee")]
    pub total_fee: Decimal,

    /// Leverage multiplier (like "5" means 5x leverage), only applicable to margin trading.
    pub leverage: Decimal,

    /// Margin mode (cross/isolated)
    #[serde(rename = "marginMode")]
    pub margin_mode: String,
}
