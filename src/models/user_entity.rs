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

/// UserEntity : Provide information about the particular entity
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserEntity {
    /// The full entity's name, concatenating the first and last name fields
    #[serde(rename = "entityName", skip_serializing_if = "Option::is_none")]
    pub entity_name: Option<String>,
    /// The type of entity assigned to the user
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<EntityType>,
    /// The first name of the user
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The last name of the user
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
}

impl UserEntity {
    /// Provide information about the particular entity
    pub fn new() -> UserEntity {
        UserEntity {
            entity_name: None,
            entity_type: None,
            first_name: None,
            last_name: None,
        }
    }
}
/// The type of entity assigned to the user
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EntityType {
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "Joint")]
    Joint,
    #[serde(rename = "ORG")]
    Org,
}

impl Default for EntityType {
    fn default() -> EntityType {
        Self::Individual
    }
}

