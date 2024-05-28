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
pub struct PreviousLossesType {
    #[serde(rename = "loss", skip_serializing_if = "Option::is_none")]
    pub loss: Option<i32>,
    #[serde(rename = "quarter", skip_serializing_if = "Option::is_none")]
    pub quarter: Option<i32>,
    #[serde(rename = "year", skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl PreviousLossesType {
    pub fn new() -> PreviousLossesType {
        PreviousLossesType {
            loss: None,
            quarter: None,
            year: None,
            currency: None,
        }
    }
}
