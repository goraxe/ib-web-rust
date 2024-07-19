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

/// OrderPreview : Projected costs and changes to margin and equity values in the account, if the order ticket were executed in full.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderPreview {
    #[serde(rename = "amount", skip_serializing_if = "Option::is_none")]
    pub amount: Option<Box<models::OrderPreviewAmount>>,
    #[serde(rename = "equity", skip_serializing_if = "Option::is_none")]
    pub equity: Option<Box<models::OrderPreviewEquity>>,
    #[serde(rename = "initial", skip_serializing_if = "Option::is_none")]
    pub initial: Option<Box<models::OrderPreviewInitial>>,
    #[serde(rename = "maintenance", skip_serializing_if = "Option::is_none")]
    pub maintenance: Option<Box<models::OrderPreviewMaintenance>>,
    #[serde(rename = "position", skip_serializing_if = "Option::is_none")]
    pub position: Option<Box<models::OrderPreviewPosition>>,
    /// Human-readable text of warning message, if applicable. Otherwise null.
    #[serde(rename = "warn", skip_serializing_if = "Option::is_none")]
    pub warn: Option<String>,
    /// Human-readable text of an error message, if applicable. Otherwise null.
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

impl OrderPreview {
    /// Projected costs and changes to margin and equity values in the account, if the order ticket were executed in full.
    pub fn new() -> OrderPreview {
        OrderPreview {
            amount: None,
            equity: None,
            initial: None,
            maintenance: None,
            position: None,
            warn: None,
            error: None,
        }
    }
}

