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
pub struct DateOfBirth {
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
}

impl DateOfBirth {
    pub fn new() -> DateOfBirth {
        DateOfBirth {
            date_of_birth: None,
        }
    }
}

