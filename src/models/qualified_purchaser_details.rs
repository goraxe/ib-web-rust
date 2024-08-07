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
pub struct QualifiedPurchaserDetails {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
}

impl QualifiedPurchaserDetails {
    pub fn new() -> QualifiedPurchaserDetails {
        QualifiedPurchaserDetails {
            code: None,
            status: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "InvestmentCompanyAct")]
    InvestmentCompanyAct,
    #[serde(rename = "DiscretionaryBasis")]
    DiscretionaryBasis,
}

impl Default for Code {
    fn default() -> Code {
        Self::InvestmentCompanyAct
    }
}

