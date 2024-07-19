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
pub struct PendingTasksResponse {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ErrorResponse>>,
    #[serde(rename = "hasError", skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    #[serde(rename = "pendingTasks", skip_serializing_if = "Option::is_none")]
    pub pending_tasks: Option<Vec<models::PendingTask>>,
    #[serde(rename = "isPendingTaskPresent", skip_serializing_if = "Option::is_none")]
    pub is_pending_task_present: Option<bool>,
}

impl PendingTasksResponse {
    pub fn new() -> PendingTasksResponse {
        PendingTasksResponse {
            error: None,
            has_error: None,
            error_description: None,
            account_id: None,
            status: None,
            description: None,
            state: None,
            pending_tasks: None,
            is_pending_task_present: None,
        }
    }
}

