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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IserverContractRulesPostRequest {
    /// Contract identifier for the interested contract.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    /// Side of the market rules apply too. Set to true for Buy Orders, set to false for Sell orders.
    #[serde(rename = "isBuy", skip_serializing_if = "Option::is_none")]
    pub is_buy: Option<bool>,
    /// Used to find trading rules related to an existing order.
    #[serde(rename = "modifyOrder", skip_serializing_if = "Option::is_none")]
    pub modify_order: Option<bool>,
    /// Specify the order identifier used for tracking a given order.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i32>,
}

impl IserverContractRulesPostRequest {
    pub fn new() -> IserverContractRulesPostRequest {
        IserverContractRulesPostRequest {
            conid: None,
            is_buy: None,
            modify_order: None,
            order_id: None,
        }
    }
}

