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

/// OrderCancelSuccess : Acknowledges IB's acceptance of the request to cancel the order. Does not report whether the cancellation can or will ultimately be enacted.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderCancelSuccess {
    /// Indicates success with value 'Request was submitted'
    #[serde(rename = "msg", skip_serializing_if = "Option::is_none")]
    pub msg: Option<Msg>,
    /// IB order ID of the order ticket requested for cancellation.
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// IB contract ID of the order ticket's instrument.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<String>,
    /// IB account to which the order was originally set to clear.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
}

impl OrderCancelSuccess {
    /// Acknowledges IB's acceptance of the request to cancel the order. Does not report whether the cancellation can or will ultimately be enacted.
    pub fn new() -> OrderCancelSuccess {
        OrderCancelSuccess {
            msg: None,
            order_id: None,
            conid: None,
            account: None,
        }
    }
}
/// Indicates success with value 'Request was submitted'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Msg {
    #[serde(rename = "Request was submitted")]
    RequestWasSubmitted,
}

impl Default for Msg {
    fn default() -> Msg {
        Self::RequestWasSubmitted
    }
}

