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
pub enum GwApiV1BankInstructionsPostRequestInstruction {
    TraditionalBankInstructionVerification(Box<models::TraditionalBankInstructionVerification>),
    AchInstruction(Box<models::AchInstruction>),
    DeleteBankInstruction(Box<models::DeleteBankInstruction>),
    PredefinedDestinationInstruction(Box<models::PredefinedDestinationInstruction>),
    EddaInstruction(Box<models::EddaInstruction>),
}

impl Default for GwApiV1BankInstructionsPostRequestInstruction {
    fn default() -> Self {
        Self::TraditionalBankInstructionVerification(Default::default())
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankInstructionCode {
    #[serde(rename = "CAACH")]
    Caach,
    #[serde(rename = "USACH")]
    Usach,
}

impl Default for BankInstructionCode {
    fn default() -> BankInstructionCode {
        Self::Caach
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AchType {
    #[serde(rename = "DEBIT")]
    Debit,
    #[serde(rename = "CREDIT")]
    Credit,
    #[serde(rename = "DEBIT_CREDIT")]
    DebitCredit,
}

impl Default for AchType {
    fn default() -> AchType {
        Self::Debit
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BankInstructionMethod {
    #[serde(rename = "LVP")]
    Lvp,
    #[serde(rename = "SEPA")]
    Sepa,
    #[serde(rename = "WIRE")]
    Wire,
    #[serde(rename = "ACH")]
    Ach,
    #[serde(rename = "CPA")]
    Cpa,
}

impl Default for BankInstructionMethod {
    fn default() -> BankInstructionMethod {
        Self::Lvp
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum DebtorIdentificationDocumentType {
    #[serde(rename = "hkId")]
    HkId,
    #[serde(rename = "passport")]
    Passport,
    #[serde(rename = "chinaId")]
    ChinaId,
}

impl Default for DebtorIdentificationDocumentType {
    fn default() -> DebtorIdentificationDocumentType {
        Self::HkId
    }
}

