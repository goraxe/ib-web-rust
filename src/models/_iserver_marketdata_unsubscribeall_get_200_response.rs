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

/// IserverMarketdataUnsubscribeallGet200Response : Indicates a successful request to unsubscribe all streams.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IserverMarketdataUnsubscribeallGet200Response {
    /// The sole key 'unsubscribed' will have boolean value true.
    #[serde(rename = "unsubscribed", skip_serializing_if = "Option::is_none")]
    pub unsubscribed: Option<bool>,
}

impl IserverMarketdataUnsubscribeallGet200Response {
    /// Indicates a successful request to unsubscribe all streams.
    pub fn new() -> IserverMarketdataUnsubscribeallGet200Response {
        IserverMarketdataUnsubscribeallGet200Response {
            unsubscribed: None,
        }
    }
}

