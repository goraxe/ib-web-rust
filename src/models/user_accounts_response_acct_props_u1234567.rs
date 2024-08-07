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
pub struct UserAccountsResponseAcctPropsU1234567 {
    #[serde(rename = "hasChildAccounts", skip_serializing_if = "Option::is_none")]
    pub has_child_accounts: Option<bool>,
    #[serde(rename = "supportsCashQty", skip_serializing_if = "Option::is_none")]
    pub supports_cash_qty: Option<bool>,
    #[serde(rename = "noFXConv", skip_serializing_if = "Option::is_none")]
    pub no_fx_conv: Option<bool>,
    #[serde(rename = "isProp", skip_serializing_if = "Option::is_none")]
    pub is_prop: Option<bool>,
    #[serde(rename = "supportsFractions", skip_serializing_if = "Option::is_none")]
    pub supports_fractions: Option<bool>,
    #[serde(rename = "allowCustomerTime", skip_serializing_if = "Option::is_none")]
    pub allow_customer_time: Option<bool>,
}

impl UserAccountsResponseAcctPropsU1234567 {
    pub fn new() -> UserAccountsResponseAcctPropsU1234567 {
        UserAccountsResponseAcctPropsU1234567 {
            has_child_accounts: None,
            supports_cash_qty: None,
            no_fx_conv: None,
            is_prop: None,
            supports_fractions: None,
            allow_customer_time: None,
        }
    }
}

