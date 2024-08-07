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
pub struct FopInstruction {
    #[serde(rename = "clientInstructionId")]
    pub client_instruction_id: String,
    #[serde(rename = "direction")]
    pub direction: Direction,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "contraBrokerAccountId")]
    pub contra_broker_account_id: String,
    #[serde(rename = "contraBrokerDtcCode")]
    pub contra_broker_dtc_code: String,
    #[serde(rename = "quantity")]
    pub quantity: f64,
    #[serde(rename = "tradingInstrument", skip_serializing_if = "Option::is_none")]
    pub trading_instrument: Option<Box<models::TradingInstrument>>,
}

impl FopInstruction {
    pub fn new(client_instruction_id: String, direction: Direction, account_id: String, contra_broker_account_id: String, contra_broker_dtc_code: String, quantity: f64) -> FopInstruction {
        FopInstruction {
            client_instruction_id,
            direction,
            account_id,
            contra_broker_account_id,
            contra_broker_dtc_code,
            quantity,
            trading_instrument: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Direction {
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "OUT")]
    Out,
}

impl Default for Direction {
    fn default() -> Direction {
        Self::In
    }
}

