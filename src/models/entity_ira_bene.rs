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
pub struct EntityIraBene {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "entityType", skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "location", skip_serializing_if = "Option::is_none")]
    pub location: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "articleOfWill", skip_serializing_if = "Option::is_none")]
    pub article_of_will: Option<String>,
}

impl EntityIraBene {
    pub fn new() -> EntityIraBene {
        EntityIraBene {
            name: None,
            entity_type: None,
            r#type: None,
            location: None,
            article_of_will: None,
        }
    }
}

