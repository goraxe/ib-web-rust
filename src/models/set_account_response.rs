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
pub struct SetAccountResponse {
    /// Confirms that the account change was set
    #[serde(rename = "set", skip_serializing_if = "Option::is_none")]
    pub set: Option<bool>,
    /// Confirms the account switched to.
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<String>,
}

impl SetAccountResponse {
    pub fn new() -> SetAccountResponse {
        SetAccountResponse {
            set: None,
            acct_id: None,
        }
    }
}

