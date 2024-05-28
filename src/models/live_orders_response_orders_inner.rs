/*
 * IB REST API
 *
 * The IB REST API reference documentation
 *
 * The version of the OpenAPI document: 2.7.0
 * Contact: api@interactivebrokers.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// LiveOrdersResponseOrdersInner : Object representing one order.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LiveOrdersResponseOrdersInner {
    /// IB account ID to which the order was placed.
    #[serde(rename = "acct", skip_serializing_if = "Option::is_none")]
    pub acct: Option<String>,
    /// Routing destination of the order ticket.
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Contract ID and routing destination in format 123456@EXCHANGE.
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// Contract ID of the order's instrument.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<String>,
    /// IB account ID to which the order was placed.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// IB-assigned order identifier.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i32>,
    /// Currency of the order ticket's Cash Quantity, if applicable.
    #[serde(rename = "cashCcy", skip_serializing_if = "Option::is_none")]
    pub cash_ccy: Option<String>,
    /// Human-readable shorthand rendering of the filled and total quantities of the order.
    #[serde(rename = "sizeAndFills", skip_serializing_if = "Option::is_none")]
    pub size_and_fills: Option<String>,
    /// Human-readable shorthand rendering of the order ticket.
    #[serde(rename = "orderDesc", skip_serializing_if = "Option::is_none")]
    pub order_desc: Option<String>,
    /// Descriptive text, or additional details that specific the instrument.
    #[serde(rename = "description1", skip_serializing_if = "Option::is_none")]
    pub description1: Option<String>,
    /// Symbol or base product code of the instrument.
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Asset class of the instrument.
    #[serde(rename = "secType", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<String>,
    /// Exchange on which the instrument is listed.
    #[serde(rename = "listingExchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// Quantity remaining to be filled in units of the instrument.
    #[serde(rename = "remainingQuantity", skip_serializing_if = "Option::is_none")]
    pub remaining_quantity: Option<String>,
    /// Quantity filled in units of the instrument.
    #[serde(rename = "filledQuantity", skip_serializing_if = "Option::is_none")]
    pub filled_quantity: Option<String>,
    /// Total size of an order in the instrument's units.
    #[serde(rename = "totalSize", skip_serializing_if = "Option::is_none")]
    pub total_size: Option<String>,
    /// Total size of a cash quantity order.
    #[serde(rename = "totalCashSize", skip_serializing_if = "Option::is_none")]
    pub total_cash_size: Option<String>,
    /// Name of business associated with instrument, or otherwise description of instrument.
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Status of the order ticket.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// IB internal order status.
    #[serde(rename = "order_ccp_status", skip_serializing_if = "Option::is_none")]
    pub order_ccp_status: Option<String>,
    /// Order type of a filled order.
    #[serde(rename = "origOrderType", skip_serializing_if = "Option::is_none")]
    pub orig_order_type: Option<String>,
    /// Indicates whether the order is supported by IB's Tax Optimization tool.
    #[serde(rename = "supportsTaxOpt", skip_serializing_if = "Option::is_none")]
    pub supports_tax_opt: Option<SupportsTaxOpt>,
    /// Time of last execution against the order in format YYMMDDhhmmss.
    #[serde(rename = "lastExecutionTime", skip_serializing_if = "Option::is_none")]
    pub last_execution_time: Option<String>,
    /// Order type of a working order ticket.
    #[serde(rename = "orderType", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Internal use. IB's UI background color in hex.
    #[serde(rename = "bgColor", skip_serializing_if = "Option::is_none")]
    pub bg_color: Option<String>,
    /// Internal use. IB's UI foreground color in hex.
    #[serde(rename = "fgColor", skip_serializing_if = "Option::is_none")]
    pub fg_color: Option<String>,
    /// Indicates whether the order ticket is an Event Trading order.
    #[serde(rename = "isEventTrading", skip_serializing_if = "Option::is_none")]
    pub is_event_trading: Option<IsEventTrading>,
    /// Price of the order, if applicable to the order type.
    #[serde(rename = "price", skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time of force of the order.
    #[serde(rename = "timeInForce", skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    /// Unix timestamp of the last execution against the order.
    #[serde(rename = "lastExecutionTime_r", skip_serializing_if = "Option::is_none")]
    pub last_execution_time_r: Option<String>,
    /// Side of the order.
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    /// Average price of fills against the order, if any.
    #[serde(rename = "avgPrice", skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<String>,
}

impl LiveOrdersResponseOrdersInner {
    /// Object representing one order.
    pub fn new() -> LiveOrdersResponseOrdersInner {
        LiveOrdersResponseOrdersInner {
            acct: None,
            exchange: None,
            conidex: None,
            conid: None,
            account: None,
            order_id: None,
            cash_ccy: None,
            size_and_fills: None,
            order_desc: None,
            description1: None,
            ticker: None,
            sec_type: None,
            listing_exchange: None,
            remaining_quantity: None,
            filled_quantity: None,
            total_size: None,
            total_cash_size: None,
            company_name: None,
            status: None,
            order_ccp_status: None,
            orig_order_type: None,
            supports_tax_opt: None,
            last_execution_time: None,
            order_type: None,
            bg_color: None,
            fg_color: None,
            is_event_trading: None,
            price: None,
            time_in_force: None,
            last_execution_time_r: None,
            side: None,
            avg_price: None,
        }
    }
}
/// Status of the order ticket.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "Inactive")]
    Inactive,
    #[serde(rename = "PendingSubmit")]
    PendingSubmit,
    #[serde(rename = "PreSubmitted")]
    PreSubmitted,
    #[serde(rename = "Submitted")]
    Submitted,
    #[serde(rename = "Filled")]
    Filled,
    #[serde(rename = "PendingCancel")]
    PendingCancel,
    #[serde(rename = "Cancelled")]
    Cancelled,
    #[serde(rename = "WarnState")]
    WarnState,
}

impl Default for Status {
    fn default() -> Status {
        Self::Inactive
    }
}
/// Indicates whether the order is supported by IB's Tax Optimization tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SupportsTaxOpt {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for SupportsTaxOpt {
    fn default() -> SupportsTaxOpt {
        Self::Variant0
    }
}
/// Indicates whether the order ticket is an Event Trading order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IsEventTrading {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "1")]
    Variant1,
}

impl Default for IsEventTrading {
    fn default() -> IsEventTrading {
        Self::Variant0
    }
}
