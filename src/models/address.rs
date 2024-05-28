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
pub struct Address {
    #[serde(rename = "street1", skip_serializing_if = "Option::is_none")]
    pub street1: Option<String>,
    #[serde(rename = "street2", skip_serializing_if = "Option::is_none")]
    pub street2: Option<String>,
    #[serde(rename = "city", skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "postalCode", skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

impl Address {
    pub fn new() -> Address {
        Address {
            street1: None,
            street2: None,
            city: None,
            state: None,
            country: None,
            postal_code: None,
        }
    }
}

