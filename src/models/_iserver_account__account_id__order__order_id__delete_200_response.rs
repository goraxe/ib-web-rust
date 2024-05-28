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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum IserverAccountAccountIdOrderOrderIdDelete200Response {
    OrderCancelSuccess(Box<models::OrderCancelSuccess>),
    OrderSubmitError(Box<models::OrderSubmitError>),
}

impl Default for IserverAccountAccountIdOrderOrderIdDelete200Response {
    fn default() -> Self {
        Self::OrderCancelSuccess(Default::default())
    }
}
/// Indicates success with value 'Request was submitted'
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Msg {
    #[serde(rename = "Request was submitted")]
    RequestWasSubmitted,
}

impl Default for Msg {
    fn default() -> Msg {
        Self::RequestWasSubmitted
    }
}
