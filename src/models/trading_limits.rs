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
pub struct TradingLimits {
    #[serde(rename = "orderValueLimits", skip_serializing_if = "Option::is_none")]
    pub order_value_limits: Option<Box<models::OrderValueLimits>>,
    #[serde(rename = "efpQuantityLimits", skip_serializing_if = "Option::is_none")]
    pub efp_quantity_limits: Option<Box<models::EfpQuantityLimits>>,
    #[serde(rename = "orderQuantityLimit", skip_serializing_if = "Option::is_none")]
    pub order_quantity_limit: Option<Vec<models::OrderQuantityLimit>>,
    #[serde(rename = "dayQuantityLimit", skip_serializing_if = "Option::is_none")]
    pub day_quantity_limit: Option<Vec<models::DayQuantityLimit>>,
}

impl TradingLimits {
    pub fn new() -> TradingLimits {
        TradingLimits {
            order_value_limits: None,
            efp_quantity_limits: None,
            order_quantity_limit: None,
            day_quantity_limit: None,
        }
    }
}

