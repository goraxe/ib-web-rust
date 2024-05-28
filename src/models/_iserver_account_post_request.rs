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
pub struct IserverAccountPostRequest {
    /// Identifier for the unique account to retrieve information from.
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<String>,
}

impl IserverAccountPostRequest {
    pub fn new() -> IserverAccountPostRequest {
        IserverAccountPostRequest {
            acct_id: None,
        }
    }
}

