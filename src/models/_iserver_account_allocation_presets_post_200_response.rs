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
pub struct IserverAccountAllocationPresetsPost200Response {
    /// Signifies that the request was successfully submitted.
    #[serde(rename = "success", skip_serializing_if = "Option::is_none")]
    pub success: Option<bool>,
}

impl IserverAccountAllocationPresetsPost200Response {
    pub fn new() -> IserverAccountAllocationPresetsPost200Response {
        IserverAccountAllocationPresetsPost200Response {
            success: None,
        }
    }
}
