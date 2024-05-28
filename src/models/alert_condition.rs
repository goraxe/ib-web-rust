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
pub struct AlertCondition {
    /// The type of condition set.
    #[serde(rename = "condition_type", skip_serializing_if = "Option::is_none")]
    pub condition_type: Option<i32>,
    /// Returns conid and exchange in the format “conid@exchange”
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// Includes relevant descriptions (if applicable).
    #[serde(rename = "contract_description_1", skip_serializing_if = "Option::is_none")]
    pub contract_description_1: Option<String>,
    /// Condition operator set for alert.
    #[serde(rename = "condition_operator", skip_serializing_if = "Option::is_none")]
    pub condition_operator: Option<String>,
    /// TriggerMethod value set.
    #[serde(rename = "condition_trigger_method", skip_serializing_if = "Option::is_none")]
    pub condition_trigger_method: Option<i32>,
    /// Condition value set.
    #[serde(rename = "condition_value", skip_serializing_if = "Option::is_none")]
    pub condition_value: Option<String>,
    /// Condition logic_bind value set.
    #[serde(rename = "condition_logic_bind", skip_serializing_if = "Option::is_none")]
    pub condition_logic_bind: Option<bool>,
    /// Condition timeZone value set.
    #[serde(rename = "condition_time_zone", skip_serializing_if = "Option::is_none")]
    pub condition_time_zone: Option<String>,
}

impl AlertCondition {
    pub fn new() -> AlertCondition {
        AlertCondition {
            condition_type: None,
            conidex: None,
            contract_description_1: None,
            condition_operator: None,
            condition_trigger_method: None,
            condition_value: None,
            condition_logic_bind: None,
            condition_time_zone: None,
        }
    }
}

