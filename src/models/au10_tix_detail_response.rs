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
pub struct Au10TixDetailResponse {
    #[serde(rename = "startDate", skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "expiryDate", skip_serializing_if = "Option::is_none")]
    pub expiry_date: Option<String>,
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ErrorResponse>>,
    #[serde(rename = "hasError", skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<i32>,
    #[serde(rename = "state", skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl Au10TixDetailResponse {
    pub fn new() -> Au10TixDetailResponse {
        Au10TixDetailResponse {
            start_date: None,
            expiry_date: None,
            error: None,
            has_error: None,
            error_description: None,
            url: None,
            external_id: None,
            entity_id: None,
            state: None,
        }
    }
}

