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
pub struct IserverScannerRunResponseContractsInner {
    /// Contract’s index in relation to the market scanner type’s sorting priority.
    #[serde(rename = "server_id", skip_serializing_if = "Option::is_none")]
    pub server_id: Option<String>,
    /// Always returned for the first contract.
    #[serde(rename = "column_name", skip_serializing_if = "Option::is_none")]
    pub column_name: Option<String>,
    /// Returns the contract’s ticker symbol.
    #[serde(rename = "symbol", skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Returns the contract ID of the contract.
    #[serde(rename = "conidex", skip_serializing_if = "Option::is_none")]
    pub conidex: Option<String>,
    /// Returns the contract ID of the contract.
    #[serde(rename = "con_id", skip_serializing_if = "Option::is_none")]
    pub con_id: Option<i32>,
    /// Internal Use Only
    #[serde(rename = "available_chart_periods", skip_serializing_if = "Option::is_none")]
    pub available_chart_periods: Option<String>,
    /// Returns the company long name.
    #[serde(rename = "company_name", skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// For derivatives like Futures, the local symbol of the contract will be returned.
    #[serde(rename = "contract_description_1", skip_serializing_if = "Option::is_none")]
    pub contract_description_1: Option<String>,
    /// Returns the primary listing exchange of the contract.
    #[serde(rename = "listing_exchange", skip_serializing_if = "Option::is_none")]
    pub listing_exchange: Option<String>,
    /// Returns the security type of the contract.
    #[serde(rename = "sec_type", skip_serializing_if = "Option::is_none")]
    pub sec_type: Option<String>,
}

impl IserverScannerRunResponseContractsInner {
    pub fn new() -> IserverScannerRunResponseContractsInner {
        IserverScannerRunResponseContractsInner {
            server_id: None,
            column_name: None,
            symbol: None,
            conidex: None,
            con_id: None,
            available_chart_periods: None,
            company_name: None,
            contract_description_1: None,
            listing_exchange: None,
            sec_type: None,
        }
    }
}

