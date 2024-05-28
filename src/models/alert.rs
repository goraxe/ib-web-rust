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

/// Alert : An array containing all alerts as separate objects.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Alert {
    /// The order id (alert id)
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i32>,
    /// The account the alert was attributed to.
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    /// The requested name for the alert.
    #[serde(rename = "alert_name", skip_serializing_if = "Option::is_none")]
    pub alert_name: Option<String>,
    /// Determines if the alert is active [1] or not [0]
    #[serde(rename = "alert_active", skip_serializing_if = "Option::is_none")]
    pub alert_active: Option<i32>,
    /// UTC-formatted time of the alert’s creation.
    #[serde(rename = "order_time", skip_serializing_if = "Option::is_none")]
    pub order_time: Option<String>,
    /// Confirms if the order is triggered or not.
    #[serde(rename = "alert_triggered", skip_serializing_if = "Option::is_none")]
    pub alert_triggered: Option<bool>,
    /// Confirms if the alert is enabled to occur more than once.
    #[serde(rename = "alert_repeatable", skip_serializing_if = "Option::is_none")]
    pub alert_repeatable: Option<i32>,
}

impl Alert {
    /// An array containing all alerts as separate objects.
    pub fn new() -> Alert {
        Alert {
            order_id: None,
            account: None,
            alert_name: None,
            alert_active: None,
            order_time: None,
            alert_triggered: None,
            alert_repeatable: None,
        }
    }
}

