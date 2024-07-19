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
pub struct GwApiV1ExternalCashTransfersPostRequest {
    #[serde(rename = "instructionType")]
    pub instruction_type: InstructionType,
    #[serde(rename = "instruction")]
    pub instruction: Box<models::GwApiV1ExternalCashTransfersPostRequestInstruction>,
}

impl GwApiV1ExternalCashTransfersPostRequest {
    pub fn new(instruction_type: InstructionType, instruction: models::GwApiV1ExternalCashTransfersPostRequestInstruction) -> GwApiV1ExternalCashTransfersPostRequest {
        GwApiV1ExternalCashTransfersPostRequest {
            instruction_type,
            instruction: Box::new(instruction),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstructionType {
    #[serde(rename = "deposit_funds")]
    DepositFunds,
    #[serde(rename = "withdraws_funds")]
    WithdrawsFunds,
}

impl Default for InstructionType {
    fn default() -> InstructionType {
        Self::DepositFunds
    }
}

