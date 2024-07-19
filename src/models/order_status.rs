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

/// OrderStatus : Object containing information about the status of an order ticket.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderStatus {
    /// Internal use only.
    #[serde(rename = "sub_type", skip_serializing_if = "Option::is_none")]
    pub sub_type: Option<String>,
    /// Internal use only. IB-assigned identifier for the status request.
    #[serde(rename = "request_id", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    /// IB-assigned meta-identifier used to associate rejected and resubmitted orders following Server Prompts.
    #[serde(rename = "server_id", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// The IB-assigned order identifier of the order, as provided in the request path.
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i32>,
    /// Contract ID and routing destination in format 123456@EXCHANGE.
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// Contract ID of the order's instrument.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<String>,
    /// Symbol of the order ticket's instrument.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Side of the order ticket.
    #[serde(rename = "side", skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    /// Human-readable description of the order's instrument.
    #[serde(rename = "contract_description_1", skip_serializing_if = "Option::is_none")]
    pub contract_description_1: Option<String>,
    /// Primary listing exchange of the order ticket's instrument.
    #[serde(rename = "listing_exchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// Internal use only.
    #[serde(rename = "option_acct", skip_serializing_if = "Option::is_none")]
    pub option_acct: Option<String>,
    /// Name of the company or asset associated with the instrument.
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Remaining unfilled size of the order ticket. Will reflect 0.0 if order is filled in full, cancelled, or otherwise resolved and no longer working.
    #[serde(rename = "size", skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    /// The total size of the order ticket.
    #[serde(rename = "total_size", skip_serializing_if = "Option::is_none")]
    pub total_size: Option<String>,
    /// The currency in which the instrument trades and executions are conducted.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The account receiving executions against this order ticket.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The order's  IB order type.
    #[serde(rename = "order_type", skip_serializing_if = "Option::is_none")]
    pub order_type: Option<String>,
    /// Cumulative filled quantity of the instrument against the order ticket.
    #[serde(rename = "cum_fill", skip_serializing_if = "Option::is_none")]
    pub cum_fill: Option<String>,
    /// Status of the order ticket.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
    /// IB internal order status.
    #[serde(rename = "order_ccp_status", skip_serializing_if = "Option::is_none")]
    pub order_ccp_status: Option<String>,
    /// Human-readable rendering of the order's status meant for presentation in UI.
    #[serde(rename = "order_status_description", skip_serializing_if = "Option::is_none")]
    pub order_status_description: Option<String>,
    /// Time in force of the order ticket.
    #[serde(rename = "tif", skip_serializing_if = "Option::is_none")]
    pub tif: Option<Tif>,
    /// Internal use. IB's UI foreground color in hex.
    #[serde(rename = "fgColor", skip_serializing_if = "Option::is_none")]
    pub fg_color: Option<String>,
    /// Internal use. IB's UI background color in hex.
    #[serde(rename = "bgColor", skip_serializing_if = "Option::is_none")]
    pub bg_color: Option<String>,
    /// Indicates whether the order ticket can be modified.
    #[serde(rename = "order_not_editable", skip_serializing_if = "Option::is_none")]
    pub order_not_editable: Option<bool>,
    /// Indicates which fields of the order ticket can be modified currently.
    #[serde(rename = "editable_fields", skip_serializing_if = "Option::is_none")]
    pub editable_fields: Option<String>,
    /// Indicates whether the order ticket can be cancelled.
    #[serde(rename = "cannot_cancel_order", skip_serializing_if = "Option::is_none")]
    pub cannot_cancel_order: Option<bool>,
    /// Indicates whether the order ticket can be deactivated.
    #[serde(rename = "deactivate_order", skip_serializing_if = "Option::is_none")]
    pub deactivate_order: Option<bool>,
    /// IB asset class identifier.
    #[serde(rename = "sec_type", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<SecType>,
    /// Internal use. Indicates chart periods available for the instrument.
    #[serde(rename = "available_chart_periods", skip_serializing_if = "Option::is_none")]
    pub available_chart_periods: Option<String>,
    /// Human-readable description of the status or current result of the order ticket, meant for UI presentation.
    #[serde(rename = "order_description", skip_serializing_if = "Option::is_none")]
    pub order_description: Option<String>,
    /// Human-readable description of the status or current result of the order ticket, meant for UI presentation. Includes instrument name.
    #[serde(rename = "order_description_with_contract", skip_serializing_if = "Option::is_none")]
    pub order_description_with_contract: Option<String>,
    /// Indicates that an alert is active for the order ticket.
    #[serde(rename = "alert_active", skip_serializing_if = "Option::is_none")]
    pub alert_active: Option<i32>,
    /// Indicates if the order ticket is hedged, and if so, in what way. 0 = No hedge, A = Attached child hedge order, B = Beta/portfolio hedge
    #[serde(rename = "child_order_type", skip_serializing_if = "Option::is_none")]
    pub child_order_type: Option<ChildOrderType>,
    /// The IB account to which the order ticket clears.
    #[serde(rename = "order_clearing_account", skip_serializing_if = "Option::is_none")]
    pub order_clearing_account: Option<String>,
    /// A string reflecting the cumulative fills and total size of the order.
    #[serde(rename = "size_and_fills", skip_serializing_if = "Option::is_none")]
    pub size_and_fills: Option<String>,
    /// Internal use. The UI-displayed price associated with a Client Portal exist strategy.
    #[serde(rename = "exit_strategy_display_price", skip_serializing_if = "Option::is_none")]
    pub exit_strategy_display_price: Option<String>,
    /// Internal use. A string describing an active Client Portal exit strategy, or the result of its execution.
    #[serde(rename = "exit_strategy_chart_description", skip_serializing_if = "Option::is_none")]
    pub exit_strategy_chart_description: Option<String>,
    /// Average price of fills against the order, if any.
    #[serde(rename = "average_price", skip_serializing_if = "Option::is_none")]
    pub average_price: Option<String>,
    /// Internal use. Indicates the availability of Client Portal exit strategy tool for the order.
    #[serde(rename = "exit_strategy_tool_availability", skip_serializing_if = "Option::is_none")]
    pub exit_strategy_tool_availability: Option<String>,
    /// Indicates whether an identical order on the opposite side can be placed.
    #[serde(rename = "allowed_duplicate_opposite", skip_serializing_if = "Option::is_none")]
    pub allowed_duplicate_opposite: Option<bool>,
    /// Time of the order's submission in format YYMMDDhhmmss.
    #[serde(rename = "order_time", skip_serializing_if = "Option::is_none")]
    pub order_time: Option<String>,
}

impl OrderStatus {
    /// Object containing information about the status of an order ticket.
    pub fn new() -> OrderStatus {
        OrderStatus {
            sub_type: None,
            request_id: None,
            server_id: None,
            order_id: None,
            conidex: None,
            conid: None,
            symbol: None,
            side: None,
            contract_description_1: None,
            listing_exchange: None,
            option_acct: None,
            company_name: None,
            size: None,
            total_size: None,
            currency: None,
            account: None,
            order_type: None,
            cum_fill: None,
            status: None,
            order_ccp_status: None,
            order_status_description: None,
            tif: None,
            fg_color: None,
            bg_color: None,
            order_not_editable: None,
            editable_fields: None,
            cannot_cancel_order: None,
            deactivate_order: None,
            sec_type: None,
            available_chart_periods: None,
            order_description: None,
            order_description_with_contract: None,
            alert_active: None,
            child_order_type: None,
            order_clearing_account: None,
            size_and_fills: None,
            exit_strategy_display_price: None,
            exit_strategy_chart_description: None,
            average_price: None,
            exit_strategy_tool_availability: None,
            allowed_duplicate_opposite: None,
            order_time: None,
        }
    }
}
/// Side of the order ticket.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Side {
    #[serde(rename = "BUY")]
    Buy,
    #[serde(rename = "SELL")]
    Sell,
}

impl Default for Side {
    fn default() -> Side {
        Self::Buy
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
/// Time in force of the order ticket.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Tif {
    #[serde(rename = "DAY")]
    Day,
    #[serde(rename = "IOC")]
    Ioc,
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "OPG")]
    Opg,
    #[serde(rename = "PAX")]
    Pax,
}

impl Default for Tif {
    fn default() -> Tif {
        Self::Day
    }
}
/// IB asset class identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecType {
    #[serde(rename = "STK")]
    Stk,
    #[serde(rename = "OPT")]
    Opt,
    #[serde(rename = "FUT")]
    Fut,
    #[serde(rename = "FOP")]
    Fop,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CRYPTO")]
    Crypto,
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "WAR")]
    War,
    #[serde(rename = "FUND")]
    Fund,
}

impl Default for SecType {
    fn default() -> SecType {
        Self::Stk
    }
}
/// Indicates if the order ticket is hedged, and if so, in what way. 0 = No hedge, A = Attached child hedge order, B = Beta/portfolio hedge
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ChildOrderType {
    #[serde(rename = "0")]
    Variant0,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "B")]
    B,
}

impl Default for ChildOrderType {
    fn default() -> ChildOrderType {
        Self::Variant0
    }
}

