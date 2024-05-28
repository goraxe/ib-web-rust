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
pub struct UserAccountsResponse {
    /// Returns an array of all accessible accountIds.
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<String>>,
    #[serde(rename = "acctProps", skip_serializing_if = "Option::is_none")]
    pub acct_props: Option<Box<models::UserAccountsResponseAcctProps>>,
    #[serde(rename = "aliases", skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Box<models::UserAccountsResponseAliases>>,
    #[serde(rename = "allowFeatures", skip_serializing_if = "Option::is_none")]
    pub allow_features: Option<Box<models::UserAccountsResponseAllowFeatures>>,
    #[serde(rename = "chartPeriods", skip_serializing_if = "Option::is_none")]
    pub chart_periods: Option<Box<models::UserAccountsResponseChartPeriods>>,
    #[serde(rename = "groups", skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(rename = "profiles", skip_serializing_if = "Option::is_none")]
    pub profiles: Option<Vec<String>>,
    #[serde(rename = "selectedAccount", skip_serializing_if = "Option::is_none")]
    pub selected_account: Option<String>,
    #[serde(rename = "serverInfo", skip_serializing_if = "Option::is_none")]
    pub server_info: Option<Box<models::UserAccountsResponseServerInfo>>,
    #[serde(rename = "sessionId", skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(rename = "isFt", skip_serializing_if = "Option::is_none")]
    pub is_ft: Option<bool>,
    #[serde(rename = "isPaper", skip_serializing_if = "Option::is_none")]
    pub is_paper: Option<bool>,
}

impl UserAccountsResponse {
    pub fn new() -> UserAccountsResponse {
        UserAccountsResponse {
            accounts: None,
            acct_props: None,
            aliases: None,
            allow_features: None,
            chart_periods: None,
            groups: None,
            profiles: None,
            selected_account: None,
            server_info: None,
            session_id: None,
            is_ft: None,
            is_paper: None,
        }
    }
}

