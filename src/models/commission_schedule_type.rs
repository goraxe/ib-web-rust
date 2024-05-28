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
pub struct CommissionScheduleType {
    #[serde(rename = "markups", skip_serializing_if = "Option::is_none")]
    pub markups: Option<Vec<models::CommissionMarkupType>>,
    #[serde(rename = "pricingStructure", skip_serializing_if = "Option::is_none")]
    pub pricing_structure: Option<PricingStructure>,
}

impl CommissionScheduleType {
    pub fn new() -> CommissionScheduleType {
        CommissionScheduleType {
            markups: None,
            pricing_structure: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PricingStructure {
    #[serde(rename = "FIXED")]
    Fixed,
    #[serde(rename = "TIERED")]
    Tiered,
}

impl Default for PricingStructure {
    fn default() -> PricingStructure {
        Self::Fixed
    }
}
