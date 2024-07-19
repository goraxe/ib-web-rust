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
pub struct RegistrationTask {
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "formName", skip_serializing_if = "Option::is_none")]
    pub form_name: Option<String>,
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<String>,
    #[serde(rename = "isRequiredForApproval", skip_serializing_if = "Option::is_none")]
    pub is_required_for_approval: Option<bool>,
    #[serde(rename = "isCompleted", skip_serializing_if = "Option::is_none")]
    pub is_completed: Option<bool>,
    #[serde(rename = "dateCompleted", skip_serializing_if = "Option::is_none")]
    pub date_completed: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl RegistrationTask {
    pub fn new() -> RegistrationTask {
        RegistrationTask {
            external_id: None,
            form_name: None,
            action: None,
            is_required_for_approval: None,
            is_completed: None,
            date_completed: None,
            state: None,
        }
    }
}
