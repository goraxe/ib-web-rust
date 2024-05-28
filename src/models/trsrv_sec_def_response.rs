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

/// TrsrvSecDefResponse : a contract's security definition
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrsrvSecDefResponse {
    #[serde(rename = "secdef", skip_serializing_if = "Option::is_none")]
    pub secdef: Option<Vec<String>>,
}

impl TrsrvSecDefResponse {
    /// a contract's security definition
    pub fn new() -> TrsrvSecDefResponse {
        TrsrvSecDefResponse {
            secdef: None,
        }
    }
}

