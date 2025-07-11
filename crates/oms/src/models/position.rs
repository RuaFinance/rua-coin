// Copyright 2025 chenjjiaa
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Position represents a user's position on a specific trading pair
///
/// Key Features:
///   - Mutable: Position size and average price update as trades occur
///   - Aggregated: Summarizes all user trading activity on a specific symbol
///   - Real-time: Reflects user's current market exposure
///
/// Typical Lifecycle:
///  1. Created when user first opens a position
///  2. Updated with subsequent trades affecting size and average price
///  3. May be deleted or marked as closed when fully closed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Unique position identifier
    pub id: String,

    /// User ID
    #[serde(rename = "userId")]
    pub user_id: String,

    /// Trading pair
    pub symbol: String,

    /// Position side ("buy" or "sell")
    #[serde(rename = "positionSide")]
    pub position_side: String,

    /// Position size
    pub quantity: Decimal,

    /// Average entry price
    #[serde(rename = "entryPrice")]
    pub entry_price: Decimal,

    /// Leverage multiplier
    pub leverage: Decimal,

    /// Liquidation price
    #[serde(rename = "liquidationPrice")]
    pub liquidation_price: Decimal,

    /// Margin type ("isolated" or "cross")
    #[serde(rename = "marginType")]
    pub margin_type: String,

    /// Margin amount
    pub margin: Decimal,

    /// Unrealized profit and loss
    #[serde(rename = "unrealizedPnl")]
    pub unrealized_pnl: Decimal,

    /// Realized profit and loss
    #[serde(rename = "realizedPnl")]
    pub realized_pnl: Decimal,

    /// Creation time
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,

    /// Update time
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}
