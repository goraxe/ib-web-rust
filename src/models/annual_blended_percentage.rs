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
pub struct AnnualBlendedPercentage {
    #[serde(rename = "blendedFrom", skip_serializing_if = "Option::is_none")]
    pub blended_from: Option<String>,
    #[serde(rename = "blendedTo", skip_serializing_if = "Option::is_none")]
    pub blended_to: Option<String>,
    #[serde(rename = "percentage", skip_serializing_if = "Option::is_none")]
    pub percentage: Option<String>,
}

impl AnnualBlendedPercentage {
    pub fn new() -> AnnualBlendedPercentage {
        AnnualBlendedPercentage {
            blended_from: None,
            blended_to: None,
            percentage: None,
        }
    }
}

