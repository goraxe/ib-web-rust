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
pub struct FyiEnableDeviceOption {
    #[serde(rename = "deviceName", skip_serializing_if = "Option::is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "deviceId", skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(rename = "uiName", skip_serializing_if = "Option::is_none")]
    pub ui_name: Option<String>,
    /// enable or disable device
    #[serde(rename = "enabled", skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

impl FyiEnableDeviceOption {
    pub fn new() -> FyiEnableDeviceOption {
        FyiEnableDeviceOption {
            device_name: None,
            device_id: None,
            ui_name: None,
            enabled: None,
        }
    }
}

