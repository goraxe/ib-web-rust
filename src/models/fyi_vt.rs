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
pub struct FyiVt {
    /// Returns 1 to state message was acknowledged.
    #[serde(rename = "V", skip_serializing_if = "Option::is_none")]
    pub v: Option<i32>,
    /// Returns the time in ms to complete the edit.
    #[serde(rename = "T", skip_serializing_if = "Option::is_none")]
    pub t: Option<i32>,
}

impl FyiVt {
    pub fn new() -> FyiVt {
        FyiVt {
            v: None,
            t: None,
        }
    }
}
