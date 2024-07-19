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
pub struct LeaveDrip {
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
}

impl LeaveDrip {
    pub fn new() -> LeaveDrip {
        LeaveDrip {
            reference_account_id: None,
        }
    }
}

