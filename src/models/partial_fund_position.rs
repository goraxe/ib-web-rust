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
pub struct PartialFundPosition {
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    #[serde(rename = "numberOfShares", skip_serializing_if = "Option::is_none")]
    pub number_of_shares: Option<i64>,
    #[serde(rename = "all", skip_serializing_if = "Option::is_none")]
    pub all: Option<bool>,
}

impl PartialFundPosition {
    pub fn new() -> PartialFundPosition {
        PartialFundPosition {
            symbol: None,
            number_of_shares: None,
            all: None,
        }
    }
}

