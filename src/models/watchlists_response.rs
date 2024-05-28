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

/// WatchlistsResponse : Object containing a successful query for watchlists saved for the username in use in the current Web API session.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct WatchlistsResponse {
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Box<models::WatchlistsResponseData>>,
    /// Internal use. Always has value 'content'.
    #[serde(rename = "action", skip_serializing_if = "Option::is_none")]
    pub action: Option<Action>,
    /// Internal use. Number of times endpoint has been visited during session.
    #[serde(rename = "MID", skip_serializing_if = "Option::is_none")]
    pub mid: Option<String>,
}

impl WatchlistsResponse {
    /// Object containing a successful query for watchlists saved for the username in use in the current Web API session.
    pub fn new() -> WatchlistsResponse {
        WatchlistsResponse {
            data: None,
            action: None,
            mid: None,
        }
    }
}
/// Internal use. Always has value 'content'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Action {
    #[serde(rename = "content")]
    Content,
}

impl Default for Action {
    fn default() -> Action {
        Self::Content
    }
}
