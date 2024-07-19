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
pub struct SummaryOfAccountBalancesResponse {
    #[serde(rename = "total", skip_serializing_if = "Option::is_none")]
    pub total: Option<Box<models::SummaryOfAccountBalancesResponseTotal>>,
    #[serde(rename = "commodities", skip_serializing_if = "Option::is_none")]
    pub commodities: Option<Box<models::SummaryOfAccountBalancesResponseCommodities>>,
    #[serde(rename = "securities", skip_serializing_if = "Option::is_none")]
    pub securities: Option<Box<models::SummaryOfAccountBalancesResponseSecurities>>,
}

impl SummaryOfAccountBalancesResponse {
    pub fn new() -> SummaryOfAccountBalancesResponse {
        SummaryOfAccountBalancesResponse {
            total: None,
            commodities: None,
            securities: None,
        }
    }
}

