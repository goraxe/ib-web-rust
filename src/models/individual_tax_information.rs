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
pub struct IndividualTaxInformation {
    #[serde(rename = "w9", skip_serializing_if = "Option::is_none")]
    pub w9: Option<Box<models::FormW9>>,
    #[serde(rename = "w8Ben", skip_serializing_if = "Option::is_none")]
    pub w8_ben: Option<Box<models::FormW8Ben>>,
    #[serde(rename = "crs", skip_serializing_if = "Option::is_none")]
    pub crs: Option<Box<models::FormCrs>>,
    #[serde(rename = "w8BenE", skip_serializing_if = "Option::is_none")]
    pub w8_ben_e: Option<Box<models::FormW8Bene>>,
}

impl IndividualTaxInformation {
    pub fn new() -> IndividualTaxInformation {
        IndividualTaxInformation {
            w9: None,
            w8_ben: None,
            crs: None,
            w8_ben_e: None,
        }
    }
}

