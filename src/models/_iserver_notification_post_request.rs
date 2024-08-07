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
pub struct IserverNotificationPostRequest {
    /// IB-assigned order identifier obtained from the ntf websocket message that delivered the server prompt.
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i32>,
    /// IB-assigned request identifier obtained from the ntf websocket message that delivered the server prompt.
    #[serde(rename = "reqId", skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
    /// The selected value from the \"options\" array delivered in the server prompt ntf websocket message.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl IserverNotificationPostRequest {
    pub fn new() -> IserverNotificationPostRequest {
        IserverNotificationPostRequest {
            order_id: None,
            req_id: None,
            text: None,
        }
    }
}

