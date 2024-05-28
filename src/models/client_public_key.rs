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
pub struct ClientPublicKey {
    #[serde(rename = "keyId")]
    pub key_id: String,
    #[serde(rename = "keyBitSize", skip_serializing_if = "Option::is_none")]
    pub key_bit_size: Option<i32>,
    #[serde(rename = "keyStatus", skip_serializing_if = "Option::is_none")]
    pub key_status: Option<String>,
    #[serde(rename = "active", skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(rename = "symmetric", skip_serializing_if = "Option::is_none")]
    pub symmetric: Option<bool>,
    #[serde(rename = "public", skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    #[serde(rename = "private", skip_serializing_if = "Option::is_none")]
    pub private: Option<bool>,
    #[serde(rename = "asymmetric", skip_serializing_if = "Option::is_none")]
    pub asymmetric: Option<bool>,
    #[serde(rename = "empty", skip_serializing_if = "Option::is_none")]
    pub empty: Option<bool>,
}

impl ClientPublicKey {
    pub fn new(key_id: String) -> ClientPublicKey {
        ClientPublicKey {
            key_id,
            key_bit_size: None,
            key_status: None,
            active: None,
            symmetric: None,
            public: None,
            private: None,
            asymmetric: None,
            empty: None,
        }
    }
}

