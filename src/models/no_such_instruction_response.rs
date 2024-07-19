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
pub struct NoSuchInstructionResponse {
    #[serde(rename = "httpStatusCode")]
    pub http_status_code: i32,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

impl NoSuchInstructionResponse {
    pub fn new(http_status_code: i32, r#type: String, title: String) -> NoSuchInstructionResponse {
        NoSuchInstructionResponse {
            http_status_code,
            r#type,
            title,
            detail: None,
        }
    }
}

