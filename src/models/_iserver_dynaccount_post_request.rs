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
pub struct IserverDynaccountPostRequest {
    /// The account ID that should be set for future requests.
    #[serde(rename = "acctId")]
    pub acct_id: String,
}

impl IserverDynaccountPostRequest {
    pub fn new(acct_id: String) -> IserverDynaccountPostRequest {
        IserverDynaccountPostRequest {
            acct_id,
        }
    }
}

