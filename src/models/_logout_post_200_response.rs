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
pub struct LogoutPost200Response {
    /// Confirms that the logout action was performed successfully.
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

impl LogoutPost200Response {
    pub fn new() -> LogoutPost200Response {
        LogoutPost200Response {
            status: None,
        }
    }
}

