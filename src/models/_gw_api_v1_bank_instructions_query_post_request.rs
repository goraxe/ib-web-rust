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
pub struct GwApiV1BankInstructionsQueryPostRequest {
    #[serde(rename = "instructionType")]
    pub instruction_type: InstructionType,
    #[serde(rename = "instruction")]
    pub instruction: Box<models::GwApiV1BankInstructionsQueryPostRequestInstruction>,
}

impl GwApiV1BankInstructionsQueryPostRequest {
    pub fn new(instruction_type: InstructionType, instruction: models::GwApiV1BankInstructionsQueryPostRequestInstruction) -> GwApiV1BankInstructionsQueryPostRequest {
        GwApiV1BankInstructionsQueryPostRequest {
            instruction_type,
            instruction: Box::new(instruction),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstructionType {
    #[serde(rename = "get_bank_instruction_name")]
    BankInstructionName,
    #[serde(rename = "get_recent_recurring_event_details")]
    RecentRecurringEventDetails,
}

impl Default for InstructionType {
    fn default() -> InstructionType {
        Self::BankInstructionName
    }
}

