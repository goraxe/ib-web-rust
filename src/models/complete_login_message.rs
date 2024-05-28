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
pub struct CompleteLoginMessage {
    #[serde(rename = "loginMessages", skip_serializing_if = "Option::is_none")]
    pub login_messages: Option<Vec<models::LoginMessage>>,
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
}

impl CompleteLoginMessage {
    pub fn new() -> CompleteLoginMessage {
        CompleteLoginMessage {
            login_messages: None,
            reference_account_id: None,
        }
    }
}
