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

/// IndividualPosition : A specific account's position in the requested conid.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct IndividualPosition {
    /// IB accountId of an account with a position in the requested conid.
    #[serde(rename = "acctId", skip_serializing_if = "Option::is_none")]
    pub acct_id: Option<String>,
    /// Comma separated all exchanges on which the instrument trades.
    #[serde(rename = "allExchanges", skip_serializing_if = "Option::is_none")]
    pub all_exchanges: Option<String>,
    /// Asset class of the requested instrument.
    #[serde(rename = "assetClass", skip_serializing_if = "Option::is_none")]
    pub asset_class: Option<String>,
    /// The account's average cost for its position.
    #[serde(rename = "avgCost", skip_serializing_if = "Option::is_none")]
    pub avg_cost: Option<f64>,
    /// The account's average price for its position.
    #[serde(rename = "avgPrice", skip_serializing_if = "Option::is_none")]
    pub avg_price: Option<f64>,
    /// Average cost in the account's base currency.
    #[serde(rename = "baseAvgCost", skip_serializing_if = "Option::is_none")]
    pub base_avg_cost: Option<f64>,
    /// Average price in the account's base currency.
    #[serde(rename = "baseAvgPrice", skip_serializing_if = "Option::is_none")]
    pub base_avg_price: Option<f64>,
    /// Market price of instrument in the account's base currency.
    #[serde(rename = "baseMktPrice", skip_serializing_if = "Option::is_none")]
    pub base_mkt_price: Option<f64>,
    /// Market value of the position in the account's base currency.
    #[serde(rename = "baseMktValue", skip_serializing_if = "Option::is_none")]
    pub base_mkt_value: Option<f64>,
    /// Realized PnL for the instrument in the account's base currency.
    #[serde(rename = "baseRealizedPnl", skip_serializing_if = "Option::is_none")]
    pub base_realized_pnl: Option<f64>,
    /// Unrealized PnL for the instrument in the account's base currency.
    #[serde(rename = "baseUnrealizedPnl", skip_serializing_if = "Option::is_none")]
    pub base_unrealized_pnl: Option<f64>,
    /// Chinese name of the instrument.
    #[serde(rename = "chineseName", skip_serializing_if = "Option::is_none")]
    pub chinese_name: Option<String>,
    #[serde(rename = "conExchMap", skip_serializing_if = "Option::is_none")]
    pub con_exch_map: Option<Vec<String>>,
    /// IB contract ID for the instrument.
    #[serde(rename = "conid", skip_serializing_if = "Option::is_none")]
    pub conid: Option<i32>,
    /// Human-readable description of the instrument.
    #[serde(rename = "contractDesc", skip_serializing_if = "Option::is_none")]
    pub contract_desc: Option<String>,
    /// Country in which the instrument is issued.
    #[serde(rename = "countryCode", skip_serializing_if = "Option::is_none")]
    pub country_code: Option<String>,
    /// Currency in which the instrument trades.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(rename = "displayRule", skip_serializing_if = "Option::is_none")]
    pub display_rule: Option<Box<models::IndividualPositionDisplayRule>>,
    #[serde(rename = "exchs", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exchs: Option<Option<serde_json::Value>>,
    /// Style of exercise for options.
    #[serde(rename = "exerciseStyle", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub exercise_style: Option<Option<String>>,
    /// Expiration of instrument, if applicable.
    #[serde(rename = "expiry", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub expiry: Option<Option<String>>,
    /// Full display name of the instrument.
    #[serde(rename = "fullName", skip_serializing_if = "Option::is_none")]
    pub full_name: Option<String>,
    /// Industry sub-categorization of the instrument.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Indicates whether instrument has options contracts available for trading at IB.
    #[serde(rename = "hasOptions", skip_serializing_if = "Option::is_none")]
    pub has_options: Option<bool>,
    /// Array containing increment rules used when pricing orders for the instrument.
    #[serde(rename = "incrementRules", skip_serializing_if = "Option::is_none")]
    pub increment_rules: Option<Vec<models::IndividualPositionIncrementRulesInner>>,
    /// Indicates whether the instrument is an Event Contract.
    #[serde(rename = "isEventContract", skip_serializing_if = "Option::is_none")]
    pub is_event_contract: Option<bool>,
    /// Indicates whether the instrument is issued in the US.
    #[serde(rename = "isUS", skip_serializing_if = "Option::is_none")]
    pub is_us: Option<bool>,
    /// Last day of trading in the instrument, if applicable. Formatted `YYYYMMDD`.
    #[serde(rename = "lastTradingDay", skip_serializing_if = "Option::is_none")]
    pub last_trading_day: Option<String>,
    /// The exchange on which the instrument is listed, or the primary exchange recognized by IB for the instrument.
    #[serde(rename = "listingExchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// Current market price of the instrument, in the instrument's currency.
    #[serde(rename = "mktPrice", skip_serializing_if = "Option::is_none")]
    pub mkt_price: Option<f64>,
    /// Current market value of the account's position in the instrument, in the instrument's currency.
    #[serde(rename = "mktValue", skip_serializing_if = "Option::is_none")]
    pub mkt_value: Option<f64>,
    /// Name of the model portfolio in which the account is invested that contributes this position.
    #[serde(rename = "model", skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Instrument's multiplier, if applicable.
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<f64>,
    /// Formal name of the entity or asset to which the instrument relates.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Maximum number of accounts that can be returned in a single request.
    #[serde(rename = "pageSize", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// Size of position in units of instrument.
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<f64>,
    /// The right of an options contract, if applicable.
    #[serde(rename = "putOrCall", skip_serializing_if = "Option::is_none")]
    pub put_or_call: Option<PutOrCall>,
    /// Realized PnL for the instrument in the instrument's currency.
    #[serde(rename = "realizedPnl", skip_serializing_if = "Option::is_none")]
    pub realized_pnl: Option<f64>,
    /// Industry sector categorization of the instrument.
    #[serde(rename = "sector", skip_serializing_if = "Option::is_none")]
    pub sector: Option<String>,
    /// Industry sub-categorization of the instrument.
    #[serde(rename = "sectorGroup", skip_serializing_if = "Option::is_none")]
    pub sector_group: Option<String>,
    /// Strike price, if applicable. Returned as string.
    #[serde(rename = "strike", skip_serializing_if = "Option::is_none")]
    pub strike: Option<String>,
    /// Symbol associated with the instrument.
    #[serde(rename = "ticker", skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    /// Time taken to retrieve position data in milliseconds.
    #[serde(rename = "time", skip_serializing_if = "Option::is_none")]
    pub time: Option<i32>,
    /// Description of instrument, used to differentiate classes, if applicable.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Contract ID of underlying instrument, if applicable.
    #[serde(rename = "undConid", skip_serializing_if = "Option::is_none")]
    pub und_conid: Option<i32>,
    /// Unrealized PnL for the instrument in the account.
    #[serde(rename = "unrealizedPnl", skip_serializing_if = "Option::is_none")]
    pub unrealized_pnl: Option<f64>,
}

impl IndividualPosition {
    /// A specific account's position in the requested conid.
    pub fn new() -> IndividualPosition {
        IndividualPosition {
            acct_id: None,
            all_exchanges: None,
            asset_class: None,
            avg_cost: None,
            avg_price: None,
            base_avg_cost: None,
            base_avg_price: None,
            base_mkt_price: None,
            base_mkt_value: None,
            base_realized_pnl: None,
            base_unrealized_pnl: None,
            chinese_name: None,
            con_exch_map: None,
            conid: None,
            contract_desc: None,
            country_code: None,
            currency: None,
            display_rule: None,
            exchs: None,
            exercise_style: None,
            expiry: None,
            full_name: None,
            group: None,
            has_options: None,
            increment_rules: None,
            is_event_contract: None,
            is_us: None,
            last_trading_day: None,
            listing_exchange: None,
            mkt_price: None,
            mkt_value: None,
            model: None,
            multiplier: None,
            name: None,
            page_size: None,
            position: None,
            put_or_call: None,
            realized_pnl: None,
            sector: None,
            sector_group: None,
            strike: None,
            ticker: None,
            time: None,
            r#type: None,
            und_conid: None,
            unrealized_pnl: None,
        }
    }
}
/// The right of an options contract, if applicable.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum PutOrCall {
    #[serde(rename = "P")]
    P,
    #[serde(rename = "C")]
    C,
}

impl Default for PutOrCall {
    fn default() -> PutOrCall {
        Self::P
    }
}

