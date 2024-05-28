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

/// RegsnapshotData : Object containing regulatory snapshot data.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegsnapshotData {
    /// IB contract ID.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    /// Contract ID and routing destination in format 123456@EXCHANGE.
    #[serde(rename = "conidEx", skip_serializing_if = "Option::is_none")]
    pub conid_ex: Option<String>,
    /// Internal use. Minimum size display increment.
    #[serde(rename = "sizeMinTick", skip_serializing_if = "Option::is_none")]
    pub size_min_tick: Option<f64>,
    /// Internal use. Exchange map code.
    #[serde(rename = "BboExchange", skip_serializing_if = "Option::is_none")]
    pub bbo_exchange: Option<String>,
    /// Indicates whether delayed data is available.
    #[serde(rename = "HasDelayed", skip_serializing_if = "Option::is_none")]
    pub has_delayed: Option<bool>,
    /// Bid price.
    #[serde(rename = "84", skip_serializing_if = "Option::is_none")]
    pub param_84: Option<String>,
    /// Ask price.
    #[serde(rename = "86", skip_serializing_if = "Option::is_none")]
    pub param_86: Option<String>,
    /// Bid size in round lots (100 shares).
    #[serde(rename = "88", skip_serializing_if = "Option::is_none")]
    pub param_88: Option<i32>,
    /// Ask size in round lots (100 shares).
    #[serde(rename = "85", skip_serializing_if = "Option::is_none")]
    pub param_85: Option<i32>,
    /// Internal use. Equivalent binary encoding of field 7068.
    #[serde(rename = "BestBidExch", skip_serializing_if = "Option::is_none")]
    pub best_bid_exch: Option<i32>,
    /// Internal use. Equivalent binary encoding of field 7057.
    #[serde(rename = "BestAskExch", skip_serializing_if = "Option::is_none")]
    pub best_ask_exch: Option<i32>,
    /// Last traded price.
    #[serde(rename = "31", skip_serializing_if = "Option::is_none")]
    pub param_31: Option<String>,
    /// Last traded size in round lots (100 shares).
    #[serde(rename = "7059", skip_serializing_if = "Option::is_none")]
    pub param_7059: Option<String>,
    /// Internal use. Equivalent binary encoding of field 7058.
    #[serde(rename = "LastExch", skip_serializing_if = "Option::is_none")]
    pub last_exch: Option<i32>,
    /// Best ask exchanges(s). String of single, capital-letter MCOIDs.
    #[serde(rename = "7057", skip_serializing_if = "Option::is_none")]
    pub param_7057: Option<String>,
    /// Best bid exchange(s). String of single, capital-letter MCOIDs.
    #[serde(rename = "7068", skip_serializing_if = "Option::is_none")]
    pub param_7068: Option<String>,
    /// Exchange of last trade. A single, capital-letter MCOID.
    #[serde(rename = "7058", skip_serializing_if = "Option::is_none")]
    pub param_7058: Option<String>,
}

impl RegsnapshotData {
    /// Object containing regulatory snapshot data.
    pub fn new() -> RegsnapshotData {
        RegsnapshotData {
            conid: None,
            conid_ex: None,
            size_min_tick: None,
            bbo_exchange: None,
            has_delayed: None,
            param_84: None,
            param_86: None,
            param_88: None,
            param_85: None,
            best_bid_exch: None,
            best_ask_exch: None,
            param_31: None,
            param_7059: None,
            last_exch: None,
            param_7057: None,
            param_7068: None,
            param_7058: None,
        }
    }
}

