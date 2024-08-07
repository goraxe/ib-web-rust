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

/// PerformanceResponseTpps : Returns the time period performance data.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct PerformanceResponseTpps {
    /// Object containing all data about tpps.
    #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<models::PerformanceResponseCpsDataInner>>,
    /// Returns the determining frequency of the data range.
    #[serde(rename = "freq", skip_serializing_if = "Option::is_none")]
    pub freq: Option<String>,
    /// Returns the dates corresponding to the frequency of data.
    #[serde(rename = "dates", skip_serializing_if = "Option::is_none")]
    pub dates: Option<Vec<String>>,
}

impl PerformanceResponseTpps {
    /// Returns the time period performance data.
    pub fn new() -> PerformanceResponseTpps {
        PerformanceResponseTpps {
            data: None,
            freq: None,
            dates: None,
        }
    }
}

