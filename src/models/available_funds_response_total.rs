/*
 * IB REST API
 *
 * The IB REST API reference documentation
 *
 * The version of the OpenAPI document: 2.9.0
 * Contact: api@interactivebrokers.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// AvailableFundsResponseTotal : total values
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AvailableFundsResponseTotal {
    /// Describes currently avialable funds in your account for trading.
    #[serde(rename = "current_available", skip_serializing_if = "Option::is_none")]
    pub current_available: Option<String>,
    /// Describes total value of the account.
    #[serde(rename = "current_excess", skip_serializing_if = "Option::is_none")]
    pub current_excess: Option<String>,
    /// Displays predicted post-expiration account value.
    #[serde(rename = "Prdctd Pst-xpry Excss", skip_serializing_if = "Option::is_none")]
    pub prdctd_pst_xpry_excss: Option<String>,
    /// This value reflects your available funds at the next margin change.
    #[serde(rename = "Lk Ahd Avlbl Fnds", skip_serializing_if = "Option::is_none")]
    pub lk_ahd_avlbl_fnds: Option<String>,
    /// * `Securities` - Equity with loan value. Look ahead maintenance margin.  * `Commodities` - Net Liquidation value. Look ahead maintenance margin. 
    #[serde(rename = "Lk Ahd Excss Lqdty", skip_serializing_if = "Option::is_none")]
    pub lk_ahd_excss_lqdty: Option<String>,
    /// Describes available funds for overnight trading.
    #[serde(rename = "overnight_available", skip_serializing_if = "Option::is_none")]
    pub overnight_available: Option<String>,
    /// Overnight refers to the window of time after the local market trading day is closed.    * `Securities` - Equivalent to regular trading hours.     * `Commodities` - Commodities Net Liquidation value. Overnight Maintenance margin. 
    #[serde(rename = "overnight_excess", skip_serializing_if = "Option::is_none")]
    pub overnight_excess: Option<String>,
    /// Describes the total buying power of the account including existing balance with margin.
    #[serde(rename = "buying_power", skip_serializing_if = "Option::is_none")]
    pub buying_power: Option<String>,
    /// Describes the total combined leverage.
    #[serde(rename = "leverage", skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    /// Describes when the next 'Look Ahead' calculation will take place.
    #[serde(rename = "Lk Ahd Nxt Chng", skip_serializing_if = "Option::is_none")]
    pub lk_ahd_nxt_chng: Option<String>,
    /// Describes the number of trades remaining before flagging the Pattern Day Trader status. \"Unlimited\" is used for existing Pattern Day Traders.
    #[serde(rename = "day_trades_left", skip_serializing_if = "Option::is_none")]
    pub day_trades_left: Option<String>,
}

impl AvailableFundsResponseTotal {
    /// total values
    pub fn new() -> AvailableFundsResponseTotal {
        AvailableFundsResponseTotal {
            current_available: None,
            current_excess: None,
            prdctd_pst_xpry_excss: None,
            lk_ahd_avlbl_fnds: None,
            lk_ahd_excss_lqdty: None,
            overnight_available: None,
            overnight_excess: None,
            buying_power: None,
            leverage: None,
            lk_ahd_nxt_chng: None,
            day_trades_left: None,
        }
    }
}

