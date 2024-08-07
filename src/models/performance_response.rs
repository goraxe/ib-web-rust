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
pub struct PerformanceResponse {
    /// Confirms if the currency type. If trading exclusively in your base currency, “base” will be returned.
    #[serde(rename = "currencyType", skip_serializing_if = "Option::is_none")]
    pub currency_type: Option<String>,
    /// Returns the data identifier (Internal Use Only).
    #[serde(rename = "rc", skip_serializing_if = "Option::is_none")]
    pub rc: Option<i32>,
    #[serde(rename = "nav", skip_serializing_if = "Option::is_none")]
    pub nav: Option<Box<models::PerformanceResponseNav>>,
    /// Returns the total data points.
    #[serde(rename = "nd", skip_serializing_if = "Option::is_none")]
    pub nd: Option<i32>,
    #[serde(rename = "cps", skip_serializing_if = "Option::is_none")]
    pub cps: Option<Box<models::PerformanceResponseCps>>,
    #[serde(rename = "tpps", skip_serializing_if = "Option::is_none")]
    pub tpps: Option<Box<models::PerformanceResponseTpps>>,
    /// Returns the request identifier, getPerformanceData.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Returns an array containing accounts reviewed.
    #[serde(rename = "included", skip_serializing_if = "Option::is_none")]
    pub included: Option<Vec<String>>,
    /// Portfolio Measure. Used to indicate TWR or MWR values returned.
    #[serde(rename = "pm", skip_serializing_if = "Option::is_none")]
    pub pm: Option<String>,
}

impl PerformanceResponse {
    pub fn new() -> PerformanceResponse {
        PerformanceResponse {
            currency_type: None,
            rc: None,
            nav: None,
            nd: None,
            cps: None,
            tpps: None,
            id: None,
            included: None,
            pm: None,
        }
    }
}

