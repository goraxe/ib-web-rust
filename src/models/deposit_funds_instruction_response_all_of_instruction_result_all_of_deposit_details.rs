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
pub struct DepositFundsInstructionResponseAllOfInstructionResultAllOfDepositDetails {
    #[serde(rename = "amount")]
    pub amount: f64,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "whenAvailable")]
    pub when_available: String,
    #[serde(rename = "availableDate", skip_serializing_if = "Option::is_none")]
    pub available_date: Option<String>,
    #[serde(rename = "additionalInfo", skip_serializing_if = "Option::is_none")]
    pub additional_info: Option<String>,
}

impl DepositFundsInstructionResponseAllOfInstructionResultAllOfDepositDetails {
    pub fn new(amount: f64, currency: String, when_available: String) -> DepositFundsInstructionResponseAllOfInstructionResultAllOfDepositDetails {
        DepositFundsInstructionResponseAllOfInstructionResultAllOfDepositDetails {
            amount,
            currency,
            when_available,
            available_date: None,
            additional_info: None,
        }
    }
}

