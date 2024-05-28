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

/// NotificationReadAcknowledgeP : Returns details about the notification read status.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct NotificationReadAcknowledgeP {
    /// Returns if the message was read (1) or unread (0).
    #[serde(rename = "R", skip_serializing_if = "Option::is_none")]
    pub r: Option<i32>,
    /// Returns the ID for the notification.
    #[serde(rename = "ID", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

impl NotificationReadAcknowledgeP {
    /// Returns details about the notification read status.
    pub fn new() -> NotificationReadAcknowledgeP {
        NotificationReadAcknowledgeP {
            r: None,
            id: None,
        }
    }
}

