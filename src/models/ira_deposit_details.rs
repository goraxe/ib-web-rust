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
pub struct IraDepositDetails {
    #[serde(rename = "depositType", skip_serializing_if = "Option::is_none")]
    pub deposit_type: Option<DepositType>,
    #[serde(rename = "taxYear", skip_serializing_if = "Option::is_none")]
    pub tax_year: Option<TaxYear>,
    #[serde(rename = "fromIraType", skip_serializing_if = "Option::is_none")]
    pub from_ira_type: Option<FromIraType>,
}

impl IraDepositDetails {
    pub fn new() -> IraDepositDetails {
        IraDepositDetails {
            deposit_type: None,
            tax_year: None,
            from_ira_type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DepositType {
    #[serde(rename = "contribution")]
    Contribution,
    #[serde(rename = "rollover")]
    Rollover,
}

impl Default for DepositType {
    fn default() -> DepositType {
        Self::Contribution
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxYear {
    #[serde(rename = "current")]
    Current,
    #[serde(rename = "prior")]
    Prior,
}

impl Default for TaxYear {
    fn default() -> TaxYear {
        Self::Current
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum FromIraType {
    #[serde(rename = "RI")]
    Ri,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RT")]
    Rt,
    #[serde(rename = "SP")]
    Sp,
    #[serde(rename = "ED")]
    Ed,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "RH")]
    Rh,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "RRSP")]
    Rrsp,
    #[serde(rename = "SRRSP")]
    Srrsp,
    #[serde(rename = "TFSA")]
    Tfsa,
    #[serde(rename = "SIMPLE")]
    Simple,
}

impl Default for FromIraType {
    fn default() -> FromIraType {
        Self::Ri
    }
}

