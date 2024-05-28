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
pub struct RegulatedMemberships {
    #[serde(rename = "regulatedMemberships", skip_serializing_if = "Option::is_none")]
    pub regulated_memberships: Option<Vec<models::RegulatedMembership>>,
}

impl RegulatedMemberships {
    pub fn new() -> RegulatedMemberships {
        RegulatedMemberships {
            regulated_memberships: None,
        }
    }
}
