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
pub struct TradingPermission {
    #[serde(rename = "assetClass", skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<AssetClass>,
    #[serde(rename = "exchangeGroup", skip_serializing_if = "Option::is_none")]
    pub exchange_group: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<Country>,
    #[serde(rename = "product", skip_serializing_if = "Option::is_none")]
    pub product: Option<Product>,
}

impl TradingPermission {
    pub fn new() -> TradingPermission {
        TradingPermission {
            asset_class: None,
            exchange_group: None,
            country: None,
            product: None,
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
pub enum Country {
    #[serde(rename = "ALL")]
    All,
    #[serde(rename = "AUSTRALIA")]
    Australia,
    #[serde(rename = "AUSTRIA")]
    Austria,
    #[serde(rename = "BELGIUM")]
    Belgium,
    #[serde(rename = "CANADA")]
    Canada,
    #[serde(rename = "FRANCE")]
    France,
    #[serde(rename = "GERMANY")]
    Germany,
    #[serde(rename = "HONG KONG")]
    HongKong,
    #[serde(rename = "ITALY")]
    Italy,
    #[serde(rename = "JAPAN")]
    Japan,
    #[serde(rename = "KOREA")]
    Korea,
    #[serde(rename = "MEXICO")]
    Mexico,
    #[serde(rename = "NORWAY")]
    Norway,
    #[serde(rename = "SINGAPORE")]
    Singapore,
    #[serde(rename = "SPAIN")]
    Spain,
    #[serde(rename = "SWEDEN")]
    Sweden,
    #[serde(rename = "SWITZERLAND")]
    Switzerland,
    #[serde(rename = "THE NETHERLANDS")]
    TheNetherlands,
    #[serde(rename = "UNITED KINGDOM")]
    UnitedKingdom,
    #[serde(rename = "UNITED STATES")]
    UnitedStates,
    #[serde(rename = "HK-CHINA")]
    HkChina,
}

impl Default for Country {
    fn default() -> Country {
        Self::All
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Product {
    #[serde(rename = "BONDS")]
    Bonds,
    #[serde(rename = "FUTURES")]
    Futures,
    #[serde(rename = "FOREX")]
    Forex,
    #[serde(rename = "FUTURES OPTIONS")]
    FuturesOptions,
    #[serde(rename = "MUTUAL FUNDS")]
    MutualFunds,
    #[serde(rename = "STOCKS")]
    Stocks,
    #[serde(rename = "SINGLE STOCK FUTURES")]
    SingleStockFutures,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "STOCK OPTIONS")]
    StockOptions,
    #[serde(rename = "WARRANTS")]
    Warrants,
}

impl Default for Product {
    fn default() -> Product {
        Self::Bonds
    }
}

