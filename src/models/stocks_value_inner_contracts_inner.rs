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

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct StocksValueInnerContractsInner {
    /// Contract ID for the specific contract.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    /// Primary exchange for the given contract.
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// States whether the contract is hosted in the United States or not.
    #[serde(rename = "isUS", skip_serializing_if = "Option::is_none")]
    pub is_us: Option<bool>,
}

impl StocksValueInnerContractsInner {
    pub fn new() -> StocksValueInnerContractsInner {
        StocksValueInnerContractsInner {
            conid: None,
            exchange: None,
            is_us: None,
        }
    }
}

