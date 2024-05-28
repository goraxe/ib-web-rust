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
pub struct SecdefSearchResponseInner {
    /// applicable for bonds
    #[serde(rename = "bondid", skip_serializing_if = "Option::is_none")]
    pub bondid: Option<i32>,
    /// Contract identifier for the unique contract.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<String>,
    /// Company Name - Exchange
    #[serde(rename = "companyHeader", skip_serializing_if = "Option::is_none")]
    pub company_header: Option<String>,
    /// Formal name of the company.
    #[serde(rename = "companyName", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Underlying ticker symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Primary exchange of the contract
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Returns if the contract is available for trading.
    #[serde(rename = "restricted", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub restricted: Option<Option<bool>>,
    /// Returns a string of dates, separated by semicolons.
    #[serde(rename = "fop", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub fop: Option<Option<String>>,
    /// Returns a string of dates, separated by semicolons.
    #[serde(rename = "opt", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub opt: Option<Option<String>>,
    /// Returns a string of dates, separated by semicolons.
    #[serde(rename = "war", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub war: Option<Option<String>>,
    #[serde(rename = "sections", skip_serializing_if = "Option::is_none")]
    pub sections: Option<Vec<models::SecdefSearchResponseInnerSectionsInner>>,
    #[serde(rename = "issuers", skip_serializing_if = "Option::is_none")]
    pub issuers: Option<Vec<models::SecdefSearchResponseInnerIssuersInner>>,
}

impl SecdefSearchResponseInner {
    pub fn new() -> SecdefSearchResponseInner {
        SecdefSearchResponseInner {
            bondid: None,
            conid: None,
            company_header: None,
            company_name: None,
            symbol: None,
            description: None,
            restricted: None,
            fop: None,
            opt: None,
            war: None,
            sections: None,
            issuers: None,
        }
    }
}
