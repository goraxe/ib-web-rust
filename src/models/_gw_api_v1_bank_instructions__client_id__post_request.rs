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
pub struct GwApiV1BankInstructionsClientIdPostRequest {
    #[serde(rename = "instructionType")]
    pub instruction_type: InstructionType,
    #[serde(rename = "instruction")]
    pub instruction: Box<models::GwApiV1BankInstructionsClientIdPostRequestInstruction>,
}

impl GwApiV1BankInstructionsClientIdPostRequest {
    pub fn new(instruction_type: InstructionType, instruction: models::GwApiV1BankInstructionsClientIdPostRequestInstruction) -> GwApiV1BankInstructionsClientIdPostRequest {
        GwApiV1BankInstructionsClientIdPostRequest {
            instruction_type,
            instruction: Box::new(instruction),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstructionType {
    #[serde(rename = "ach_instruction")]
    AchInstruction,
    #[serde(rename = "traditional_bank_instruction_verification")]
    TraditionalBankInstructionVerification,
    #[serde(rename = "delete_instruction")]
    DeleteInstruction,
    #[serde(rename = "pre_dest_instruction")]
    PreDestInstruction,
    #[serde(rename = "edda_instruction")]
    EddaInstruction,
}

impl Default for InstructionType {
    fn default() -> InstructionType {
        Self::AchInstruction
    }
}

