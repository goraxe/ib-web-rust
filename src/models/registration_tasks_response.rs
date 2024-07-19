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
pub struct RegistrationTasksResponse {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ErrorResponse>>,
    #[serde(rename = "hasError", skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "isRegistrationTaskPresent", skip_serializing_if = "Option::is_none")]
    pub is_registration_task_present: Option<bool>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "registrationTasks", skip_serializing_if = "Option::is_none")]
    pub registration_tasks: Option<Vec<models::RegistrationTask>>,
}

impl RegistrationTasksResponse {
    pub fn new() -> RegistrationTasksResponse {
        RegistrationTasksResponse {
            error: None,
            has_error: None,
            error_description: None,
            account_id: None,
            is_registration_task_present: None,
            status: None,
            description: None,
            state: None,
            registration_tasks: None,
        }
    }
}
