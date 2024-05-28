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
pub struct PublicCompanyInfoType {
    #[serde(rename = "exchangeTradedOn", skip_serializing_if = "Option::is_none")]
    pub exchange_traded_on: Option<String>,
    #[serde(rename = "quotedSymbol", skip_serializing_if = "Option::is_none")]
    pub quoted_symbol: Option<String>,
}

impl PublicCompanyInfoType {
    pub fn new() -> PublicCompanyInfoType {
        PublicCompanyInfoType {
            exchange_traded_on: None,
            quoted_symbol: None,
        }
    }
}
