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
pub struct ContractRulesIncrementRulesInner {
    /// If the current mark price of the instrument is at or above the lower edge, the given increment value is used for order prices.
    #[serde(rename = "lowerEdge", skip_serializing_if = "Option::is_none")]
    pub lower_edge: Option<i32>,
    /// The price of the instrument must be submitted as a mulitple of the increment value.
    #[serde(rename = "increment", skip_serializing_if = "Option::is_none")]
    pub increment: Option<i32>,
}

impl ContractRulesIncrementRulesInner {
    pub fn new() -> ContractRulesIncrementRulesInner {
        ContractRulesIncrementRulesInner {
            lower_edge: None,
            increment: None,
        }
    }
}

