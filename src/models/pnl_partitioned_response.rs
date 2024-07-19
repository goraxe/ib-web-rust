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
pub struct PnlPartitionedResponse {
    #[serde(rename = "upnl", skip_serializing_if = "Option::is_none")]
    pub upnl: Option<Box<models::PnlPartitionedResponseUpnl>>,
}

impl PnlPartitionedResponse {
    pub fn new() -> PnlPartitionedResponse {
        PnlPartitionedResponse {
            upnl: None,
        }
    }
}

