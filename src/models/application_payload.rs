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
pub struct ApplicationPayload {
    #[serde(rename = "application", skip_serializing_if = "Option::is_none")]
    pub application: Option<Box<models::Application>>,
}

impl ApplicationPayload {
    pub fn new() -> ApplicationPayload {
        ApplicationPayload {
            application: None,
        }
    }
}

