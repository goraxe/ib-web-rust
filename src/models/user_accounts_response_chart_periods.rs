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
pub struct UserAccountsResponseChartPeriods {
    #[serde(rename = "STK", skip_serializing_if = "Option::is_none")]
    pub stk: Option<Vec<String>>,
    #[serde(rename = "CFD", skip_serializing_if = "Option::is_none")]
    pub cfd: Option<Vec<String>>,
    #[serde(rename = "OPT", skip_serializing_if = "Option::is_none")]
    pub opt: Option<Vec<String>>,
    #[serde(rename = "FOP", skip_serializing_if = "Option::is_none")]
    pub fop: Option<Vec<String>>,
    #[serde(rename = "WAR", skip_serializing_if = "Option::is_none")]
    pub war: Option<Vec<String>>,
    #[serde(rename = "IOPT", skip_serializing_if = "Option::is_none")]
    pub iopt: Option<Vec<String>>,
    #[serde(rename = "FUT", skip_serializing_if = "Option::is_none")]
    pub fut: Option<Vec<String>>,
    #[serde(rename = "CASH", skip_serializing_if = "Option::is_none")]
    pub cash: Option<Vec<String>>,
    #[serde(rename = "IND", skip_serializing_if = "Option::is_none")]
    pub ind: Option<Vec<String>>,
    #[serde(rename = "BOND", skip_serializing_if = "Option::is_none")]
    pub bond: Option<Vec<String>>,
    #[serde(rename = "FUND", skip_serializing_if = "Option::is_none")]
    pub fund: Option<Vec<String>>,
    #[serde(rename = "CMDTY", skip_serializing_if = "Option::is_none")]
    pub cmdty: Option<Vec<String>>,
    #[serde(rename = "PHYSS", skip_serializing_if = "Option::is_none")]
    pub physs: Option<Vec<String>>,
    #[serde(rename = "CRYPTO", skip_serializing_if = "Option::is_none")]
    pub crypto: Option<Vec<String>>,
}

impl UserAccountsResponseChartPeriods {
    pub fn new() -> UserAccountsResponseChartPeriods {
        UserAccountsResponseChartPeriods {
            stk: None,
            cfd: None,
            opt: None,
            fop: None,
            war: None,
            iopt: None,
            fut: None,
            cash: None,
            ind: None,
            bond: None,
            fund: None,
            cmdty: None,
            physs: None,
            crypto: None,
        }
    }
}

