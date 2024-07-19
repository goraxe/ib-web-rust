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

/// IserverQuestionsSuppressResetPost200Response : Confirms successful removal of suppression.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IserverQuestionsSuppressResetPost200Response {
    /// Confirms the successful removal of suppression. Always returns \"Submitted\".
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl IserverQuestionsSuppressResetPost200Response {
    /// Confirms successful removal of suppression.
    pub fn new() -> IserverQuestionsSuppressResetPost200Response {
        IserverQuestionsSuppressResetPost200Response {
            status: None,
        }
    }
}

