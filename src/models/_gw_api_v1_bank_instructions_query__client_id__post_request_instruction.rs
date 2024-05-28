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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GwApiV1BankInstructionsQueryClientIdPostRequestInstruction {
    GetBankInstructionDetailsQuery(Box<models::GetBankInstructionDetailsQuery>),
    GetRecentRecurringEventDetails(Box<models::GetRecentRecurringEventDetails>),
}

impl Default for GwApiV1BankInstructionsQueryClientIdPostRequestInstruction {
    fn default() -> Self {
        Self::GetBankInstructionDetailsQuery(Default::default())
    }
}
