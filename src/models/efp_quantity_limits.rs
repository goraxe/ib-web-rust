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
pub struct EfpQuantityLimits {
    #[serde(rename = "maxNominalEfpPerOrder", skip_serializing_if = "Option::is_none")]
    pub max_nominal_efp_per_order: Option<i32>,
    #[serde(rename = "maxNetEfpTrades", skip_serializing_if = "Option::is_none")]
    pub max_net_efp_trades: Option<i32>,
    #[serde(rename = "maxGrossEfpTrades", skip_serializing_if = "Option::is_none")]
    pub max_gross_efp_trades: Option<i32>,
}

impl EfpQuantityLimits {
    pub fn new() -> EfpQuantityLimits {
        EfpQuantityLimits {
            max_nominal_efp_per_order: None,
            max_net_efp_trades: None,
            max_gross_efp_trades: None,
        }
    }
}

