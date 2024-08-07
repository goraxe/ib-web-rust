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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GwApiV1AccountsGet200Response {
    FileDetailsResponse(Box<models::FileDetailsResponse>),
    ResponseFileResponse(Box<models::ResponseFileResponse>),
}

impl Default for GwApiV1AccountsGet200Response {
    fn default() -> Self {
        Self::FileDetailsResponse(Default::default())
    }
}

