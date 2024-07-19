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
pub struct DepositNotification {
    #[serde(rename = "checkDetails", skip_serializing_if = "Option::is_none")]
    pub check_details: Option<Box<models::CheckDetails>>,
    #[serde(rename = "wireDetails", skip_serializing_if = "Option::is_none")]
    pub wire_details: Option<Box<models::WireDetails>>,
    #[serde(rename = "achDetails", skip_serializing_if = "Option::is_none")]
    pub ach_details: Option<Box<models::AchDetails>>,
    #[serde(rename = "iraDepositDetails", skip_serializing_if = "Option::is_none")]
    pub ira_deposit_details: Option<Box<models::IraDepositDetails>>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    #[serde(rename = "ibAccount", skip_serializing_if = "Option::is_none")]
    pub ib_account: Option<String>,
}

impl DepositNotification {
    pub fn new() -> DepositNotification {
        DepositNotification {
            check_details: None,
            wire_details: None,
            ach_details: None,
            ira_deposit_details: None,
            r#type: None,
            amount: None,
            currency: None,
            ib_account: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "CHECK")]
    Check,
    #[serde(rename = "WIRE")]
    Wire,
    #[serde(rename = "ACH")]
    Ach,
    #[serde(rename = "SKIP_DEPOSIT")]
    SkipDeposit,
}

impl Default for Type {
    fn default() -> Type {
        Self::Check
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "MXN")]
    Mxn,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "NZD")]
    Nzd,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "CZK")]
    Czk,
    #[serde(rename = "CNH")]
    Cnh,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "ILS")]
    Ils,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "SGD")]
    Sgd,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "ZAR")]
    Zar,
    #[serde(rename = "AED")]
    Aed,
    #[serde(rename = "KRW")]
    Krw,
    #[serde(rename = "SAR")]
    Sar,
    #[serde(rename = "TRY")]
    Try,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::Usd
    }
}

