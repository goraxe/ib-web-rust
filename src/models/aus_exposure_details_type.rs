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
pub struct AusExposureDetailsType {
    #[serde(rename = "ausExposureRelationship", skip_serializing_if = "Option::is_none")]
    pub aus_exposure_relationship: Option<String>,
    #[serde(rename = "personName", skip_serializing_if = "Option::is_none")]
    pub person_name: Option<String>,
    #[serde(rename = "licenseNumber", skip_serializing_if = "Option::is_none")]
    pub license_number: Option<i32>,
}

impl AusExposureDetailsType {
    pub fn new() -> AusExposureDetailsType {
        AusExposureDetailsType {
            aus_exposure_relationship: None,
            person_name: None,
            license_number: None,
        }
    }
}

