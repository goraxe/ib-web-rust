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
pub struct NonDisclosedDetail {
    #[serde(rename = "tradeDate")]
    pub trade_date: String,
    #[serde(rename = "settleDate")]
    pub settle_date: String,
    #[serde(rename = "psetBic", skip_serializing_if = "Option::is_none")]
    pub pset_bic: Option<String>,
    #[serde(rename = "reagDeagBic", skip_serializing_if = "Option::is_none")]
    pub reag_deag_bic: Option<String>,
    #[serde(rename = "buyerSellBic", skip_serializing_if = "Option::is_none")]
    pub buyer_sell_bic: Option<String>,
    #[serde(rename = "memberAccountId", skip_serializing_if = "Option::is_none")]
    pub member_account_id: Option<String>,
    #[serde(rename = "safeKeepingAccountId", skip_serializing_if = "Option::is_none")]
    pub safe_keeping_account_id: Option<String>,
}

impl NonDisclosedDetail {
    pub fn new(trade_date: String, settle_date: String) -> NonDisclosedDetail {
        NonDisclosedDetail {
            trade_date,
            settle_date,
            pset_bic: None,
            reag_deag_bic: None,
            buyer_sell_bic: None,
            member_account_id: None,
            safe_keeping_account_id: None,
        }
    }
}

