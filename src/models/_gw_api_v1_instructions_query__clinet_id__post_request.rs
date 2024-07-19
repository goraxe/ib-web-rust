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
pub struct GwApiV1InstructionsQueryClinetIdPostRequest {
    #[serde(rename = "instructionType")]
    pub instruction_type: InstructionType,
    #[serde(rename = "instruction")]
    pub instruction: Box<models::GetRecentInstructionsQuery>,
}

impl GwApiV1InstructionsQueryClinetIdPostRequest {
    pub fn new(instruction_type: InstructionType, instruction: models::GetRecentInstructionsQuery) -> GwApiV1InstructionsQueryClinetIdPostRequest {
        GwApiV1InstructionsQueryClinetIdPostRequest {
            instruction_type,
            instruction: Box::new(instruction),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstructionType {
    #[serde(rename = "recent_instructions")]
    RecentInstructions,
}

impl Default for InstructionType {
    fn default() -> InstructionType {
        Self::RecentInstructions
    }
}
