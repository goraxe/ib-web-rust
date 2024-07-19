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
pub struct LocalTaxForm {
    #[serde(rename = "taxAuthority", skip_serializing_if = "Option::is_none")]
    pub tax_authority: Option<TaxAuthority>,
    #[serde(rename = "qualified", skip_serializing_if = "Option::is_none")]
    pub qualified: Option<bool>,
    #[serde(rename = "treatyCountry", skip_serializing_if = "Option::is_none")]
    pub treaty_country: Option<String>,
}

impl LocalTaxForm {
    pub fn new() -> LocalTaxForm {
        LocalTaxForm {
            tax_authority: None,
            qualified: None,
            treaty_country: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TaxAuthority {
    #[serde(rename = "ISRAEL_TA")]
    IsraelTa,
    #[serde(rename = "CANADA_TA")]
    CanadaTa,
    #[serde(rename = "RUSSIA_TA")]
    RussiaTa,
    #[serde(rename = "SWEDEN_TA")]
    SwedenTa,
    #[serde(rename = "AUSTRALIA_TA")]
    AustraliaTa,
}

impl Default for TaxAuthority {
    fn default() -> TaxAuthority {
        Self::IsraelTa
    }
}

