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
pub struct Ownership {
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<i32>,
}

impl Ownership {
    pub fn new() -> Ownership {
        Ownership {
            percentage: None,
        }
    }
}

