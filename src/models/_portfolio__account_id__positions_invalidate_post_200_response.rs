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
pub struct PortfolioAccountIdPositionsInvalidatePost200Response {
    /// Indicates success or failure of request to discard cached positions.
    #[serde(rename = "message", skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

impl PortfolioAccountIdPositionsInvalidatePost200Response {
    pub fn new() -> PortfolioAccountIdPositionsInvalidatePost200Response {
        PortfolioAccountIdPositionsInvalidatePost200Response {
            message: None,
        }
    }
}

