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
pub struct InternalServerErrorResponse {
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "status")]
    pub status: i32,
}

impl InternalServerErrorResponse {
    pub fn new(r#type: String, title: String, status: i32) -> InternalServerErrorResponse {
        InternalServerErrorResponse {
            r#type,
            title,
            status,
        }
    }
}

