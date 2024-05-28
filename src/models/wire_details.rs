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
pub struct WireDetails {
    #[serde(rename = "bankName", skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "bankAcctNumber", skip_serializing_if = "Option::is_none")]
    pub bank_acct_number: Option<String>,
    #[serde(rename = "bankCode", skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    #[serde(rename = "routingNumber", skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    #[serde(rename = "instruction", skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    #[serde(rename = "referenceNumber", skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
}

impl WireDetails {
    pub fn new() -> WireDetails {
        WireDetails {
            bank_name: None,
            bank_acct_number: None,
            bank_code: None,
            routing_number: None,
            instruction: None,
            country_code: None,
            reference_number: None,
        }
    }
}
