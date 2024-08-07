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
pub struct AlertActivationRequest {
    /// The alert Identifier
    #[serde(rename = "alertId")]
    pub alert_id: i32,
    /// Set whether or not the alert should be active (1) or inactive (0).
    #[serde(rename = "alertActive")]
    pub alert_active: i32,
}

impl AlertActivationRequest {
    pub fn new(alert_id: i32, alert_active: i32) -> AlertActivationRequest {
        AlertActivationRequest {
            alert_id,
            alert_active,
        }
    }
}

