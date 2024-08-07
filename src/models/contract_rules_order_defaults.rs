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

/// ContractRulesOrderDefaults : Indicates default order type for the given security type.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContractRulesOrderDefaults {
    #[serde(rename = "LMT", skip_serializing_if = "Option::is_none")]
    pub lmt: Option<Box<models::ContractRulesOrderDefaultsLmt>>,
}

impl ContractRulesOrderDefaults {
    /// Indicates default order type for the given security type.
    pub fn new() -> ContractRulesOrderDefaults {
        ContractRulesOrderDefaults {
            lmt: None,
        }
    }
}

