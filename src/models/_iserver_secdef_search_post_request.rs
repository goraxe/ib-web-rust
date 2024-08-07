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
pub struct IserverSecdefSearchPostRequest {
    /// The ticker symbol, bond issuer type, or company name of the equity you are looking to trade.
    #[serde(rename = "symbol")]
    pub symbol: String,
    /// Available underlying security types:   * `STK` - Represents an underlying as a Stock security type.   * `IND` - Represents an underlying as an Index security type.   * `BOND` - Represents an underlying as a Bond security type. 
    #[serde(rename = "secType", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<SecType>,
    /// Denotes if the symbol value is the ticker symbol or part of the company's name.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<bool>,
    #[serde(rename = "more", skip_serializing_if = "Option::is_none")]
    pub more: Option<bool>,
    /// fund search
    #[serde(rename = "fund", skip_serializing_if = "Option::is_none")]
    pub fund: Option<bool>,
    #[serde(rename = "fundFamilyConidEx", skip_serializing_if = "Option::is_none")]
    pub fund_family_conid_ex: Option<String>,
    /// pattern search
    #[serde(rename = "pattern", skip_serializing_if = "Option::is_none")]
    pub pattern: Option<bool>,
    #[serde(rename = "referrer", skip_serializing_if = "Option::is_none")]
    pub referrer: Option<String>,
}

impl IserverSecdefSearchPostRequest {
    pub fn new(symbol: String) -> IserverSecdefSearchPostRequest {
        IserverSecdefSearchPostRequest {
            symbol,
            sec_type: None,
            name: None,
            more: None,
            fund: None,
            fund_family_conid_ex: None,
            pattern: None,
            referrer: None,
        }
    }
}
/// Available underlying security types:   * `STK` - Represents an underlying as a Stock security type.   * `IND` - Represents an underlying as an Index security type.   * `BOND` - Represents an underlying as a Bond security type. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SecType {
    #[serde(rename = "STK")]
    Stk,
    #[serde(rename = "IND")]
    Ind,
    #[serde(rename = "BOND")]
    Bond,
}

impl Default for SecType {
    fn default() -> SecType {
        Self::Stk
    }
}

