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

/// SsoValidateResponseFeatures : Returns supported features such as bonds and option trading.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SsoValidateResponseFeatures {
    /// Returns the connecting environment to distinguish production or paper.
    #[serde(rename = "envs", skip_serializing_if = "Option::is_none")]
    pub envs: Option<String>,
    /// Internal Use Only
    #[serde(rename = "wlms", skip_serializing_if = "Option::is_none")]
    pub wlms: Option<bool>,
    /// Returns if realtime market data is available
    #[serde(rename = "realtime", skip_serializing_if = "Option::is_none")]
    pub realtime: Option<bool>,
    /// Returns if bonds can be traded.
    #[serde(rename = "bond", skip_serializing_if = "Option::is_none")]
    pub bond: Option<bool>,
    /// Returns if option chains can be retrieved in the account.
    #[serde(rename = "optionChains", skip_serializing_if = "Option::is_none")]
    pub option_chains: Option<bool>,
    /// Returns if trading calendars are enabled
    #[serde(rename = "calendar", skip_serializing_if = "Option::is_none")]
    pub calendar: Option<bool>,
    /// Internal Use Only
    #[serde(rename = "newMf", skip_serializing_if = "Option::is_none")]
    pub new_mf: Option<bool>,
}

impl SsoValidateResponseFeatures {
    /// Returns supported features such as bonds and option trading.
    pub fn new() -> SsoValidateResponseFeatures {
        SsoValidateResponseFeatures {
            envs: None,
            wlms: None,
            realtime: None,
            bond: None,
            option_chains: None,
            calendar: None,
            new_mf: None,
        }
    }
}

