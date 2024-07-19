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
pub struct InternalPositionTransferInstruction {
    #[serde(rename = "clientInstructionId")]
    pub client_instruction_id: String,
    #[serde(rename = "sourceAccountId")]
    pub source_account_id: String,
    #[serde(rename = "targetAccountId")]
    pub target_account_id: String,
    #[serde(rename = "transferQuantity")]
    pub transfer_quantity: f64,
    #[serde(rename = "tradingInstrument")]
    pub trading_instrument: Box<models::TradingInstrument>,
}

impl InternalPositionTransferInstruction {
    pub fn new(client_instruction_id: String, source_account_id: String, target_account_id: String, transfer_quantity: f64, trading_instrument: models::TradingInstrument) -> InternalPositionTransferInstruction {
        InternalPositionTransferInstruction {
            client_instruction_id,
            source_account_id,
            target_account_id,
            transfer_quantity,
            trading_instrument: Box::new(trading_instrument),
        }
    }
}

