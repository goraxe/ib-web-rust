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
pub struct UpdateBcan {
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
    #[serde(rename = "bcan", skip_serializing_if = "Option::is_none")]
    pub bcan: Option<String>,
    #[serde(rename = "ceNumber", skip_serializing_if = "Option::is_none")]
    pub ce_number: Option<String>,
}

impl UpdateBcan {
    pub fn new() -> UpdateBcan {
        UpdateBcan {
            reference_account_id: None,
            bcan: None,
            ce_number: None,
        }
    }
}

