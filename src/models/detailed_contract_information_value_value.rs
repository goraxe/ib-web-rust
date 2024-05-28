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

/// DetailedContractInformationValueValue : Returns the performance data for the given period value.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct DetailedContractInformationValueValue {
    /// Net asset value data for the account or consolidated accounts. NAV data is not applicable to benchmarks.
    #[serde(rename = "nav", skip_serializing_if = "Option::is_none")]
    pub nav: Option<Vec<f64>>,
    /// Returns the object containing the Cumulative performance data. Correlates to the same index position of data reutnred by the \"nav\" field.
    #[serde(rename = "cps", skip_serializing_if = "Option::is_none")]
    pub cps: Option<Vec<f64>>,
    /// Returns the determining frequency of the data range.
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<String>,
    /// Returns the dates corresponding to the frequency of data.
    #[serde(rename = "dates", skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<String>>,
    #[serde(rename = "startNav", skip_serializing_if = "Option::is_none")]
    pub start_nav: Option<Box<models::DetailedContractInformationValueValueStartNav>>,
}

impl DetailedContractInformationValueValue {
    /// Returns the performance data for the given period value.
    pub fn new() -> DetailedContractInformationValueValue {
        DetailedContractInformationValueValue {
            nav: None,
            cps: None,
            freq: None,
            dates: None,
            start_nav: None,
        }
    }
}

