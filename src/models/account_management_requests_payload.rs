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
pub struct AccountManagementRequestsPayload {
    #[serde(rename = "accountManagementRequests", skip_serializing_if = "Option::is_none")]
    pub account_management_requests: Option<Box<models::AccountManagementRequests>>,
}

impl AccountManagementRequestsPayload {
    pub fn new() -> AccountManagementRequestsPayload {
        AccountManagementRequestsPayload {
            account_management_requests: None,
        }
    }
}

