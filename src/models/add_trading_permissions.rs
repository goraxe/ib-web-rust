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
pub struct AddTradingPermissions {
    #[serde(rename = "tradingPermissions", skip_serializing_if = "Option::is_none")]
    pub trading_permissions: Option<Vec<models::TradingPermission>>,
    #[serde(rename = "documentSubmission", skip_serializing_if = "Option::is_none")]
    pub document_submission: Option<Box<models::DocumentSubmission>>,
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
}

impl AddTradingPermissions {
    pub fn new() -> AddTradingPermissions {
        AddTradingPermissions {
            trading_permissions: None,
            document_submission: None,
            reference_account_id: None,
        }
    }
}
