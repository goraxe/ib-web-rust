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
pub struct IserverContractConidInfoAndRulesGet200Response {
    /// Classification of Financial Instrument codes
    #[serde(rename = "cfi_code", skip_serializing_if = "Option::is_none")]
    pub cfi_code: Option<String>,
    /// Underlying symbol
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Returns the CUSIP for the given instrument. Only used in BOND trading.
    #[serde(rename = "cusip", skip_serializing_if = "Option::is_none")]
    pub cusip: Option<String>,
    /// Returns the expiration month of the contract.
    #[serde(rename = "expiry_full", skip_serializing_if = "Option::is_none")]
    pub expiry_full: Option<String>,
    /// Indicates the contract identifier of the given contract.
    #[serde(rename = "con_id", skip_serializing_if = "Option::is_none")]
    pub con_id: Option<i32>,
    /// Indicates the final maturity date of the given contract.
    #[serde(rename = "maturity_date", skip_serializing_if = "Option::is_none")]
    pub maturity_date: Option<String>,
    /// Specific group of companies or businesses.
    #[serde(rename = "industry", skip_serializing_if = "Option::is_none")]
    pub industry: Option<String>,
    /// Asset class of the instrument.
    #[serde(rename = "instrument_type", skip_serializing_if = "Option::is_none")]
    pub instrument_type: Option<String>,
    /// Designated trading class of the contract.
    #[serde(rename = "trading_class", skip_serializing_if = "Option::is_none")]
    pub trading_class: Option<String>,
    /// Comma separated list of support exchanges or trading venues.
    #[serde(rename = "valid_exchanges", skip_serializing_if = "Option::is_none")]
    pub valid_exchanges: Option<String>,
    /// Allowed to sell shares you own.
    #[serde(rename = "allow_sell_long", skip_serializing_if = "Option::is_none")]
    pub allow_sell_long: Option<bool>,
    /// Indicates if the contract supports zero commission trading.
    #[serde(rename = "is_zero_commission_security", skip_serializing_if = "Option::is_none")]
    pub is_zero_commission_security: Option<bool>,
    /// Contract’s symbol from primary exchange. For options it is the OCC symbol.
    #[serde(rename = "local_symbol", skip_serializing_if = "Option::is_none")]
    pub local_symbol: Option<String>,
    #[serde(rename = "contract_clarification_type", skip_serializing_if = "Option::is_none")]
    pub contract_clarification_type: Option<String>,
    #[serde(rename = "classifier", skip_serializing_if = "Option::is_none")]
    pub classifier: Option<String>,
    /// Base currency contract is traded in.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Indicates the display name of the contract, as shown with Client Portal.
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// Underlying contract identifier for the requested contract.
    #[serde(rename = "underlying_con_id", skip_serializing_if = "Option::is_none")]
    pub underlying_con_id: Option<i32>,
    /// Indicates if the contract can be traded outside regular trading hours or not.
    #[serde(rename = "r_t_h", skip_serializing_if = "Option::is_none")]
    pub r_t_h: Option<bool>,
    /// Indicates the multiplier of the contract.
    #[serde(rename = "multiplier", skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<String>,
    /// Indicates the issuer of the underlying.
    #[serde(rename = "underlying_issuer", skip_serializing_if = "Option::is_none")]
    pub underlying_issuer: Option<String>,
    /// Indicates the year and month the contract expires.
    #[serde(rename = "contract_month", skip_serializing_if = "Option::is_none")]
    pub contract_month: Option<String>,
    /// Indicates the name of the company or index.
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Indicates if the contract can be smart routed or not.
    #[serde(rename = "smart_available", skip_serializing_if = "Option::is_none")]
    pub smart_available: Option<bool>,
    /// Indicates the primary exchange for which the contract can be traded.
    #[serde(rename = "exchange", skip_serializing_if = "Option::is_none")]
    pub exchange: Option<String>,
    /// Indicates the industry category of the instrument.
    #[serde(rename = "category", skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "rules", skip_serializing_if = "Option::is_none")]
    pub rules: Option<Box<models::ContractRules>>,
}

impl IserverContractConidInfoAndRulesGet200Response {
    pub fn new() -> IserverContractConidInfoAndRulesGet200Response {
        IserverContractConidInfoAndRulesGet200Response {
            cfi_code: None,
            symbol: None,
            cusip: None,
            expiry_full: None,
            con_id: None,
            maturity_date: None,
            industry: None,
            instrument_type: None,
            trading_class: None,
            valid_exchanges: None,
            allow_sell_long: None,
            is_zero_commission_security: None,
            local_symbol: None,
            contract_clarification_type: None,
            classifier: None,
            currency: None,
            text: None,
            underlying_con_id: None,
            r_t_h: None,
            multiplier: None,
            underlying_issuer: None,
            contract_month: None,
            company_name: None,
            smart_available: None,
            exchange: None,
            category: None,
            rules: None,
        }
    }
}

