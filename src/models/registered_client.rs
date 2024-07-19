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
pub struct RegisteredClient {
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    #[serde(rename = "client_id", skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(rename = "client_name", skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(rename = "client_type", skip_serializing_if = "Option::is_none")]
    pub client_type: Option<ClientType>,
    #[serde(rename = "client_status", skip_serializing_if = "Option::is_none")]
    pub client_status: Option<ClientStatus>,
    #[serde(rename = "redirect_uris", skip_serializing_if = "Option::is_none")]
    pub redirect_uris: Option<Vec<String>>,
    #[serde(rename = "jwks", skip_serializing_if = "Option::is_none")]
    pub jwks: Option<Box<models::ClientPublicKeySet>>,
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(rename = "client_uri", skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    #[serde(rename = "logo_uri", skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(rename = "policy_uri", skip_serializing_if = "Option::is_none")]
    pub policy_uri: Option<String>,
    #[serde(rename = "account_id", skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "csid", skip_serializing_if = "Option::is_none")]
    pub csid: Option<String>,
    #[serde(rename = "created_at", skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

impl RegisteredClient {
    pub fn new() -> RegisteredClient {
        RegisteredClient {
            id: None,
            client_id: None,
            client_name: None,
            client_type: None,
            client_status: None,
            redirect_uris: None,
            jwks: None,
            description: None,
            client_uri: None,
            logo_uri: None,
            policy_uri: None,
            account_id: None,
            csid: None,
            created_at: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientType {
    #[serde(rename = "CONFIDENTIAL")]
    Confidential,
    #[serde(rename = "PUBLIC")]
    Public,
    #[serde(rename = "TEST")]
    Test,
}

impl Default for ClientType {
    fn default() -> ClientType {
        Self::Confidential
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClientStatus {
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "ACTIVE")]
    Active,
    #[serde(rename = "REVOKED")]
    Revoked,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}

impl Default for ClientStatus {
    fn default() -> ClientStatus {
        Self::Requested
    }
}

