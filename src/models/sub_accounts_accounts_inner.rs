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
pub struct SubAccountsAccountsInner {
    /// Contains Net liquidation and available equity of the given account Id.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<String>>,
    /// Returns the account ID affiliated with the balance data.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SubAccountsAccountsInner {
    pub fn new() -> SubAccountsAccountsInner {
        SubAccountsAccountsInner {
            data: None,
            name: None,
        }
    }
}

