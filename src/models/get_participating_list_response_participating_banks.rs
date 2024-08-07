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
pub struct GetParticipatingListResponseParticipatingBanks {
    #[serde(rename = "institutionName")]
    pub institution_name: String,
    #[serde(rename = "clearingCode")]
    pub clearing_code: String,
    #[serde(rename = "BIC")]
    pub bic: String,
}

impl GetParticipatingListResponseParticipatingBanks {
    pub fn new(institution_name: String, clearing_code: String, bic: String) -> GetParticipatingListResponseParticipatingBanks {
        GetParticipatingListResponseParticipatingBanks {
            institution_name,
            clearing_code,
            bic,
        }
    }
}

