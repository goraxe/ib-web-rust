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
pub struct PortfolioSummaryColumnprioS {
    /// Numerical data for the associated key.
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// The currency in which the value of the 'amount' field is denominated.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<f64>,
    /// Indicates whether the associated key's value does not exist, as opposed to a value of zero.
    #[serde(rename = "isNull", skip_serializing_if = "Option::is_none")]
    pub is_null: Option<bool>,
    /// severity
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    /// Unix epoch timestamp of returned data in milliseconds.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    /// String and boolean (non-numerical) data for the associated key.
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl PortfolioSummaryColumnprioS {
    pub fn new() -> PortfolioSummaryColumnprioS {
        PortfolioSummaryColumnprioS {
            amount: None,
            currency: None,
            is_null: None,
            severity: None,
            timestamp: None,
            value: None,
        }
    }
}

