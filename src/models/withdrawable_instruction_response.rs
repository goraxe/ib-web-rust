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
pub struct WithdrawableInstructionResponse {
    #[serde(rename = "instructionSetId")]
    pub instruction_set_id: f64,
    #[serde(rename = "instructionResults", skip_serializing_if = "Option::is_none")]
    pub instruction_results: Option<Vec<models::WithdrawableInstructionResult>>,
}

impl WithdrawableInstructionResponse {
    pub fn new(instruction_set_id: f64) -> WithdrawableInstructionResponse {
        WithdrawableInstructionResponse {
            instruction_set_id,
            instruction_results: None,
        }
    }
}

