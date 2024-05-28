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
pub struct GwApiV1FeeTemplatesClientIdPostRequest {
    #[serde(rename = "instructionType")]
    pub instruction_type: InstructionType,
    #[serde(rename = "instruction")]
    pub instruction: Box<models::ApplyFeeTemplateInstruction>,
}

impl GwApiV1FeeTemplatesClientIdPostRequest {
    pub fn new(instruction_type: InstructionType, instruction: models::ApplyFeeTemplateInstruction) -> GwApiV1FeeTemplatesClientIdPostRequest {
        GwApiV1FeeTemplatesClientIdPostRequest {
            instruction_type,
            instruction: Box::new(instruction),
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InstructionType {
    #[serde(rename = "apply_fee_template")]
    ApplyFeeTemplate,
}

impl Default for InstructionType {
    fn default() -> InstructionType {
        Self::ApplyFeeTemplate
    }
}

