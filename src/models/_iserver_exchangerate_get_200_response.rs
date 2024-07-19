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
pub struct IserverExchangerateGet200Response {
    /// Returns the exchange rate for the currency pair.
    #[serde(rename = "rate", skip_serializing_if = "Option::is_none")]
    pub rate: Option<f64>,
}

impl IserverExchangerateGet200Response {
    pub fn new() -> IserverExchangerateGet200Response {
        IserverExchangerateGet200Response {
            rate: None,
        }
    }
}

