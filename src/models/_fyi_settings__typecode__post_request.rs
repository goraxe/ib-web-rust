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
pub struct FyiSettingsTypecodePostRequest {
    /// the notification should be enabled or disabled.
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl FyiSettingsTypecodePostRequest {
    pub fn new() -> FyiSettingsTypecodePostRequest {
        FyiSettingsTypecodePostRequest {
            enabled: None,
        }
    }
}

