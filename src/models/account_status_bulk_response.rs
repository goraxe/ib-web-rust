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
pub struct AccountStatusBulkResponse {
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<models::AccountStatusResponse>>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ErrorResponse>>,
    #[serde(rename = "hasError", skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
}

impl AccountStatusBulkResponse {
    pub fn new() -> AccountStatusBulkResponse {
        AccountStatusBulkResponse {
            accounts: None,
            error: None,
            has_error: None,
            error_description: None,
        }
    }
}

