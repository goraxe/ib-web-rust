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
pub struct WithdrawableInstructionResult {
    #[serde(rename = "clientInstructionId")]
    pub client_instruction_id: String,
    #[serde(rename = "instructionType")]
    pub instruction_type: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "withdrawable_amount", skip_serializing_if = "Option::is_none")]
    pub withdrawable_amount: Option<f64>,
    #[serde(rename = "withdrawable_amount_currency", skip_serializing_if = "Option::is_none")]
    pub withdrawable_amount_currency: Option<String>,
}

impl WithdrawableInstructionResult {
    pub fn new(client_instruction_id: String, instruction_type: String, account_id: String, status: String) -> WithdrawableInstructionResult {
        WithdrawableInstructionResult {
            client_instruction_id,
            instruction_type,
            account_id,
            status,
            withdrawable_amount: None,
            withdrawable_amount_currency: None,
        }
    }
}

