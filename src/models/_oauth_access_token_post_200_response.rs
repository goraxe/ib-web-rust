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
pub struct OauthAccessTokenPost200Response {
    /// Indicates whether the authorizing username is paper or not.
    #[serde(rename = "is_true", skip_serializing_if = "Option::is_none")]
    pub is_true: Option<bool>,
    /// Permanent OAuth access token for the consumer with respect to the authorizing username. 20 character hex value.
    #[serde(rename = "oauth_token", skip_serializing_if = "Option::is_none")]
    pub oauth_token: Option<String>,
    /// OAuth token secret value. Base64-encoded string.
    #[serde(rename = "oauth_token_secret", skip_serializing_if = "Option::is_none")]
    pub oauth_token_secret: Option<String>,
}

impl OauthAccessTokenPost200Response {
    pub fn new() -> OauthAccessTokenPost200Response {
        OauthAccessTokenPost200Response {
            is_true: None,
            oauth_token: None,
            oauth_token_secret: None,
        }
    }
}

