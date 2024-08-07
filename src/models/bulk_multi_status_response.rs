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
pub struct BulkMultiStatusResponse {
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: i32,
    #[serde(rename = "instructionSetId")]
    pub instruction_set_id: f64,
    #[serde(rename = "instructionResults", skip_serializing_if = "Option::is_none")]
    pub instruction_results: Option<Vec<String>>,
}

impl BulkMultiStatusResponse {
    pub fn new(http_status_code: i32, instruction_set_id: f64) -> BulkMultiStatusResponse {
        BulkMultiStatusResponse {
            http_status_code,
            instruction_set_id,
            instruction_results: None,
        }
    }
}

