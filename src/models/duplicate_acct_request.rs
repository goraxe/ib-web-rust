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
pub struct DuplicateAcctRequest {
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
    #[serde(rename = "numberOfDuplicates", skip_serializing_if = "Option::is_none")]
    pub number_of_duplicates: Option<i32>,
}

impl DuplicateAcctRequest {
    pub fn new() -> DuplicateAcctRequest {
        DuplicateAcctRequest {
            reference_account_id: None,
            number_of_duplicates: None,
        }
    }
}
