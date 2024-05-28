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
pub struct NotificationsInner {
    /// Notification date as an epoch string.
    #[serde(rename = "D", skip_serializing_if = "Option::is_none")]
    pub d: Option<String>,
    /// Unique way to reference the notification.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// FYI code, we can use it to find whether the disclaimer is accepted or not in settings
    #[serde(rename = "FC", skip_serializing_if = "Option::is_none")]
    pub fc: Option<String>,
    /// Content of notification.
    #[serde(rename = "MD", skip_serializing_if = "Option::is_none")]
    pub md: Option<String>,
    /// Title of notification.
    #[serde(rename = "MS", skip_serializing_if = "Option::is_none")]
    pub ms: Option<String>,
    /// Return if the notification was read or not. Value Format: 0: Disabled; 1: Enabled. 
    #[serde(rename = "R", skip_serializing_if = "Option::is_none")]
    pub r: Option<String>,
}

impl NotificationsInner {
    pub fn new() -> NotificationsInner {
        NotificationsInner {
            d: None,
            id: None,
            fc: None,
            md: None,
            ms: None,
            r: None,
        }
    }
}

