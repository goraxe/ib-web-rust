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
pub struct IserverMarketdataUnsubscribePostRequest {
    /// The IB contract ID of the instrument whose market data feed is to be unsubscribed.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
}

impl IserverMarketdataUnsubscribePostRequest {
    pub fn new() -> IserverMarketdataUnsubscribePostRequest {
        IserverMarketdataUnsubscribePostRequest {
            conid: None,
        }
    }
}
