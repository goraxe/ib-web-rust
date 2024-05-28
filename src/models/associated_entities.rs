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
pub struct AssociatedEntities {
    #[serde(rename = "associatedIndividuals", skip_serializing_if = "Option::is_none")]
    pub associated_individuals: Option<Vec<models::AssociatedIndividual>>,
    #[serde(rename = "associatedEntities", skip_serializing_if = "Option::is_none")]
    pub associated_entities: Option<Vec<models::AssociatedEntity>>,
}

impl AssociatedEntities {
    pub fn new() -> AssociatedEntities {
        AssociatedEntities {
            associated_individuals: None,
            associated_entities: None,
        }
    }
}
