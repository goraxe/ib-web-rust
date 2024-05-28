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

/// AccountAttributesParent : Describes account relations in partitioned or multiplexed (segemented) account structures.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct AccountAttributesParent {
    /// Money manager client.
    #[serde(rename = "mmc", skip_serializing_if = "Option::is_none")]
    pub mmc: Option<Vec<String>>,
    /// Account ID of the parent account in a multiplex account structure.
    #[serde(rename = "accountId", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Indicates that the given account is a multiplex child account.
    #[serde(rename = "isMChild", skip_serializing_if = "Option::is_none")]
    pub is_m_child: Option<bool>,
    /// Indicates that the given account is itself a multiplex parent account.
    #[serde(rename = "isMParent", skip_serializing_if = "Option::is_none")]
    pub is_m_parent: Option<bool>,
    /// Indicates that the account is a multiplex account.
    #[serde(rename = "isMultiplex", skip_serializing_if = "Option::is_none")]
    pub is_multiplex: Option<bool>,
}

impl AccountAttributesParent {
    /// Describes account relations in partitioned or multiplexed (segemented) account structures.
    pub fn new() -> AccountAttributesParent {
        AccountAttributesParent {
            mmc: None,
            account_id: None,
            is_m_child: None,
            is_m_parent: None,
            is_multiplex: None,
        }
    }
}
