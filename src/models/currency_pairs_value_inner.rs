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
pub struct CurrencyPairsValueInner {
    /// The official symbol of the given currency pair.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// The official contract identifier of the given currency pair.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    /// Returns the symbol counterpart.
    #[serde(rename = "ccyPair", skip_serializing_if = "Option::is_none")]
    pub ccy_pair: Option<i32>,
}

impl CurrencyPairsValueInner {
    pub fn new() -> CurrencyPairsValueInner {
        CurrencyPairsValueInner {
            symbol: None,
            conid: None,
            ccy_pair: None,
        }
    }
}
