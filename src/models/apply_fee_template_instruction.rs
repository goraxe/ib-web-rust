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
pub struct ApplyFeeTemplateInstruction {
    #[serde(rename = "clientInstructionId")]
    pub client_instruction_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "templateName")]
    pub template_name: String,
}

impl ApplyFeeTemplateInstruction {
    pub fn new(client_instruction_id: String, account_id: String, template_name: String) -> ApplyFeeTemplateInstruction {
        ApplyFeeTemplateInstruction {
            client_instruction_id,
            account_id,
            template_name,
        }
    }
}

