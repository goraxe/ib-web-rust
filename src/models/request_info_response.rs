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
pub struct RequestInfoResponse {
    #[serde(rename = "requestId", skip_serializing_if = "Option::is_none")]
    pub request_id: Option<i64>,
    #[serde(rename = "executedAt", skip_serializing_if = "Option::is_none")]
    pub executed_at: Option<String>,
}

impl RequestInfoResponse {
    pub fn new() -> RequestInfoResponse {
        RequestInfoResponse {
            request_id: None,
            executed_at: None,
        }
    }
}
