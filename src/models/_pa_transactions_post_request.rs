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
pub struct PaTransactionsPostRequest {
    /// Include each account ID as a string to receive data for.
    #[serde(rename = "acctIds", skip_serializing_if = "Option::is_none")]
    pub acct_ids: Option<Vec<String>>,
    /// Include contract ID to receive data for.  Conids may be passed as integers or strings. Only supports one contract id at a time. 
    #[serde(rename = "conids", skip_serializing_if = "Option::is_none")]
    pub conids: Option<Vec<String>>,
    /// Define the currency to display price amounts with.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Specify the number of days to receive transaction data for.
    #[serde(rename = "days", skip_serializing_if = "Option::is_none")]
    pub days: Option<i32>,
}

impl PaTransactionsPostRequest {
    pub fn new() -> PaTransactionsPostRequest {
        PaTransactionsPostRequest {
            acct_ids: None,
            conids: None,
            currency: None,
            days: None,
        }
    }
}

