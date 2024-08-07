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

/// LedgerValue : Object describing the account's balances in its base currency, by asset class and account segments. Will be duplicated by another object in response bearing the currency's name.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct LedgerValue {
    /// The Account ID of the requested account.
    #[serde(rename = "acctcode", skip_serializing_if = "Option::is_none")]
    pub acctcode: Option<String>,
    /// The given account's cash balance in this currency.
    #[serde(rename = "cashbalance", skip_serializing_if = "Option::is_none")]
    pub cashbalance: Option<f64>,
    /// The given account's cash balance in its dedicated forex segment in this currency, if applicable.
    #[serde(rename = "cashbalancefxsegment", skip_serializing_if = "Option::is_none")]
    pub cashbalancefxsegment: Option<f64>,
    /// Market value of the given account's commodity positions in this currency.
    #[serde(rename = "commoditymarketvalue", skip_serializing_if = "Option::is_none")]
    pub commoditymarketvalue: Option<f64>,
    /// Market value of the given account's corporate bond positions in this currency.
    #[serde(rename = "corporatebondsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub corporatebondsmarketvalue: Option<f64>,
    /// Three-letter name of the currency reflected by this object, or 'BASE' for the account's base currency.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// The given account's receivable (not yet disbursed) dividend balance in this currency.
    #[serde(rename = "dividends", skip_serializing_if = "Option::is_none")]
    pub dividends: Option<f64>,
    /// Exchange rate of this currency relative to the account's base currency.
    #[serde(rename = "exchangerate", skip_serializing_if = "Option::is_none")]
    pub exchangerate: Option<i32>,
    /// The value of the given account's mutual fund holdings in this currency.
    #[serde(rename = "funds", skip_serializing_if = "Option::is_none")]
    pub funds: Option<f64>,
    /// Market value of the given account's futures positions in this currency.
    #[serde(rename = "futuremarketvalue", skip_serializing_if = "Option::is_none")]
    pub futuremarketvalue: Option<f64>,
    /// Market value of the given account's futures options positions in this currency.
    #[serde(rename = "futureoptionmarketvalue", skip_serializing_if = "Option::is_none")]
    pub futureoptionmarketvalue: Option<f64>,
    /// PnL of the given account's futures positions in this currency.
    #[serde(rename = "futuresonlypnl", skip_serializing_if = "Option::is_none")]
    pub futuresonlypnl: Option<f64>,
    /// The given account's receivable interest balance in this currency.
    #[serde(rename = "interest", skip_serializing_if = "Option::is_none")]
    pub interest: Option<f64>,
    /// Market value of the given account's issuer options positions in this currency.
    #[serde(rename = "issueroptionsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub issueroptionsmarketvalue: Option<f64>,
    /// Identifies the nature of data. Always takes values 'LedgerList'.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<Key>,
    /// The value of the given account's money market fund holdings in this currency.
    #[serde(rename = "moneyfunds", skip_serializing_if = "Option::is_none")]
    pub moneyfunds: Option<f64>,
    /// The given account's net liquidation value of positions in this currency.
    #[serde(rename = "netliquidationvalue", skip_serializing_if = "Option::is_none")]
    pub netliquidationvalue: Option<f64>,
    /// The given account's realized PnL for positions in this currency.
    #[serde(rename = "realizedpnl", skip_serializing_if = "Option::is_none")]
    pub realizedpnl: Option<f64>,
    /// Additional identifier of the currency reflected in this object. Always matches 'currency' field.
    #[serde(rename = "secondkey", skip_serializing_if = "Option::is_none")]
    pub secondkey: Option<String>,
    #[serde(rename = "sessionid", skip_serializing_if = "Option::is_none")]
    pub sessionid: Option<i32>,
    /// The given account's settled cash balance in this currency.
    #[serde(rename = "settledcash", skip_serializing_if = "Option::is_none")]
    pub settledcash: Option<f64>,
    #[serde(rename = "severity", skip_serializing_if = "Option::is_none")]
    pub severity: Option<i32>,
    /// Market value of the given account's stock positions in this currency.
    #[serde(rename = "stockmarketvalue", skip_serializing_if = "Option::is_none")]
    pub stockmarketvalue: Option<f64>,
    /// Market value of the given account's stock options positions in this currency.
    #[serde(rename = "stockoptionmarketvalue", skip_serializing_if = "Option::is_none")]
    pub stockoptionmarketvalue: Option<f64>,
    /// Market value of the given account's treasury bill positions in this currency.
    #[serde(rename = "tbillsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub tbillsmarketvalue: Option<f64>,
    /// Market value of the given account's treasury bond positions in this currency.
    #[serde(rename = "tbondsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub tbondsmarketvalue: Option<f64>,
    /// Timestamp of retrievable of this account ledger data.
    #[serde(rename = "timestamp", skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i32>,
    /// The given account's unrealied PnL for positions in this currency.
    #[serde(rename = "unrealizedpnl", skip_serializing_if = "Option::is_none")]
    pub unrealizedpnl: Option<f64>,
    /// Market value of the given account's warrant positions in this currency.
    #[serde(rename = "warrantsmarketvalue", skip_serializing_if = "Option::is_none")]
    pub warrantsmarketvalue: Option<f64>,
}

impl LedgerValue {
    /// Object describing the account's balances in its base currency, by asset class and account segments. Will be duplicated by another object in response bearing the currency's name.
    pub fn new() -> LedgerValue {
        LedgerValue {
            acctcode: None,
            cashbalance: None,
            cashbalancefxsegment: None,
            commoditymarketvalue: None,
            corporatebondsmarketvalue: None,
            currency: None,
            dividends: None,
            exchangerate: None,
            funds: None,
            futuremarketvalue: None,
            futureoptionmarketvalue: None,
            futuresonlypnl: None,
            interest: None,
            issueroptionsmarketvalue: None,
            key: None,
            moneyfunds: None,
            netliquidationvalue: None,
            realizedpnl: None,
            secondkey: None,
            sessionid: None,
            settledcash: None,
            severity: None,
            stockmarketvalue: None,
            stockoptionmarketvalue: None,
            tbillsmarketvalue: None,
            tbondsmarketvalue: None,
            timestamp: None,
            unrealizedpnl: None,
            warrantsmarketvalue: None,
        }
    }
}
/// Identifies the nature of data. Always takes values 'LedgerList'.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Key {
    #[serde(rename = "LedgerList")]
    LedgerList,
}

impl Default for Key {
    fn default() -> Key {
        Self::LedgerList
    }
}

