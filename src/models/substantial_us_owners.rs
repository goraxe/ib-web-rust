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
pub struct SubstantialUsOwners {
    #[serde(rename = "substantialUsOwner", skip_serializing_if = "Option::is_none")]
    pub substantial_us_owner: Option<Vec<models::SubstantialUsOwner>>,
}

impl SubstantialUsOwners {
    pub fn new() -> SubstantialUsOwners {
        SubstantialUsOwners {
            substantial_us_owner: None,
        }
    }
}
