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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GwApiV1ExternalCashTransfersPostRequestInstruction {
    DepositFundsInstruction(Box<models::DepositFundsInstruction>),
    WithdrawFundInstruction(Box<models::WithdrawFundInstruction>),
}

impl Default for GwApiV1ExternalCashTransfersPostRequestInstruction {
    fn default() -> Self {
        Self::DepositFundsInstruction(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankInstructionMethod {
    #[serde(rename = "ACH")]
    Ach,
    #[serde(rename = "WIRE")]
    Wire,
    #[serde(rename = "CHECK")]
    Check,
}

impl Default for BankInstructionMethod {
    fn default() -> BankInstructionMethod {
        Self::Ach
    }
}

