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
pub struct ExternalPositionTransfer {
    #[serde(rename = "clientInstructionId")]
    pub client_instruction_id: String,
    #[serde(rename = "type")]
    pub r#type: Type,
    #[serde(rename = "subtype")]
    pub subtype: Subtype,
    #[serde(rename = "brokerId")]
    pub broker_id: String,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "accountAtBroker")]
    pub account_at_broker: String,
    #[serde(rename = "sourceIRAType")]
    pub source_ira_type: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "signature")]
    pub signature: String,
}

impl ExternalPositionTransfer {
    pub fn new(client_instruction_id: String, r#type: Type, subtype: Subtype, broker_id: String, broker_name: String, account_at_broker: String, source_ira_type: String, account_id: String, signature: String) -> ExternalPositionTransfer {
        ExternalPositionTransfer {
            client_instruction_id,
            r#type,
            subtype,
            broker_id,
            broker_name,
            account_at_broker,
            source_ira_type,
            account_id,
            signature,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FULL")]
    Full,
}

impl Default for Type {
    fn default() -> Type {
        Self::Full
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Subtype {
    #[serde(rename = "ACATS")]
    Acats,
}

impl Default for Subtype {
    fn default() -> Subtype {
        Self::Acats
    }
}

