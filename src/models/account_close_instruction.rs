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
pub struct AccountCloseInstruction {
    #[serde(rename = "clientInstructionId")]
    pub client_instruction_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "closeReason", skip_serializing_if = "Option::is_none")]
    pub close_reason: Option<String>,
    #[serde(rename = "withdrawalInfo", skip_serializing_if = "Option::is_none")]
    pub withdrawal_info: Option<Box<models::AccountCloseInstructionWithdrawalInfo>>,
}

impl AccountCloseInstruction {
    pub fn new(client_instruction_id: String, account_id: String) -> AccountCloseInstruction {
        AccountCloseInstruction {
            client_instruction_id,
            account_id,
            close_reason: None,
            withdrawal_info: None,
        }
    }
}

