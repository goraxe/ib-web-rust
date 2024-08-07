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
pub struct InvalidArgument {
    #[serde(rename = "field")]
    pub field: String,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl InvalidArgument {
    pub fn new(field: String) -> InvalidArgument {
        InvalidArgument {
            field,
            description: None,
        }
    }
}

