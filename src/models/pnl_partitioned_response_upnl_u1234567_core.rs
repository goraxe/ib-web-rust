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

/// PnlPartitionedResponseUpnlU1234567Core : The account or model's Profit and Loss.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PnlPartitionedResponseUpnlU1234567Core {
    /// Returns the positional value of the returned account. Always returns 1 for individual accounts.
    #[serde(rename = "rowType", skip_serializing_if = "Option::is_none")]
    pub row_type: Option<i32>,
    /// Daily PnL for the specified account profile.
    #[serde(rename = "dpl", skip_serializing_if = "Option::is_none")]
    pub dpl: Option<i32>,
    /// Net Liquidity for the specified account profile.
    #[serde(rename = "nl", skip_serializing_if = "Option::is_none")]
    pub nl: Option<i32>,
    /// Unrealized PnL for the specified account profile.
    #[serde(rename = "upl", skip_serializing_if = "Option::is_none")]
    pub upl: Option<i32>,
    /// Excess Liquidity for the specified account profile.
    #[serde(rename = "el", skip_serializing_if = "Option::is_none")]
    pub el: Option<i32>,
    /// Margin value for the specified account profile.
    #[serde(rename = "mv", skip_serializing_if = "Option::is_none")]
    pub mv: Option<i32>,
}

impl PnlPartitionedResponseUpnlU1234567Core {
    /// The account or model's Profit and Loss.
    pub fn new() -> PnlPartitionedResponseUpnlU1234567Core {
        PnlPartitionedResponseUpnlU1234567Core {
            row_type: None,
            dpl: None,
            nl: None,
            upl: None,
            el: None,
            mv: None,
        }
    }
}

