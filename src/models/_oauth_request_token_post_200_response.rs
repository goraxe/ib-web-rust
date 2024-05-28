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
pub struct OauthRequestTokenPost200Response {
    /// Temporary OAuth access token. 20 character hex value.
    #[serde(rename = "oauth_token", skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
}

impl OauthRequestTokenPost200Response {
    pub fn new() -> OauthRequestTokenPost200Response {
        OauthRequestTokenPost200Response {
            oauth_token: None,
        }
    }
}
