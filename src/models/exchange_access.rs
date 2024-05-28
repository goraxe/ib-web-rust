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
pub struct ExchangeAccess {
    #[serde(rename = "assetClass", skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<AssetClass>,
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<Exchange>,
}

impl ExchangeAccess {
    pub fn new() -> ExchangeAccess {
        ExchangeAccess {
            asset_class: None,
            exchange: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AssetClass {
    #[serde(rename = "BILL")]
    Bill,
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "CASH")]
    Cash,
    #[serde(rename = "CFD")]
    Cfd,
    #[serde(rename = "COMB")]
    Comb,
    #[serde(rename = "FOP")]
    Fop,
    #[serde(rename = "FUND")]
    Fund,
    #[serde(rename = "FUT")]
    Fut,
    #[serde(rename = "OPT")]
    Opt,
    #[serde(rename = "SSF")]
    Ssf,
    #[serde(rename = "STK")]
    Stk,
    #[serde(rename = "WAR")]
    War,
    #[serde(rename = "MRGN")]
    Mrgn,
}

impl Default for AssetClass {
    fn default() -> AssetClass {
        Self::Bill
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Exchange {
    #[serde(rename = "NYSE")]
    Nyse,
    #[serde(rename = "AMEX")]
    Amex,
    #[serde(rename = "NASDAQ")]
    Nasdaq,
    #[serde(rename = "CBOE")]
    Cboe,
    #[serde(rename = "ISE")]
    Ise,
    #[serde(rename = "BOX")]
    Box,
    #[serde(rename = "PHLX")]
    Phlx,
    #[serde(rename = "PSE")]
    Pse,
}

impl Default for Exchange {
    fn default() -> Exchange {
        Self::Nyse
    }
}
