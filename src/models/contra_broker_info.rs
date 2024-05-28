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
pub struct ContraBrokerInfo {
    #[serde(rename = "accountType")]
    pub account_type: String,
    #[serde(rename = "brokerName")]
    pub broker_name: String,
    #[serde(rename = "depositoryId")]
    pub depository_id: String,
    #[serde(rename = "brokerAccountId")]
    pub broker_account_id: String,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "contactName", skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    #[serde(rename = "contactEmail")]
    pub contact_email: String,
    #[serde(rename = "contactPhone")]
    pub contact_phone: String,
}

impl ContraBrokerInfo {
    pub fn new(account_type: String, broker_name: String, depository_id: String, broker_account_id: String, country: String, contact_email: String, contact_phone: String) -> ContraBrokerInfo {
        ContraBrokerInfo {
            account_type,
            broker_name,
            depository_id,
            broker_account_id,
            country,
            contact_name: None,
            contact_email,
            contact_phone,
        }
    }
}

