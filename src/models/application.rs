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
pub struct Application {
    #[serde(rename = "customer", skip_serializing_if = "Option::is_none")]
    pub customer: Option<Box<models::Customer>>,
    #[serde(rename = "accounts", skip_serializing_if = "Option::is_none")]
    pub accounts: Option<Vec<models::Account>>,
    #[serde(rename = "users", skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<models::User>>,
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<models::Document>>,
    #[serde(rename = "additionalAccounts", skip_serializing_if = "Option::is_none")]
    pub additional_accounts: Option<Vec<models::AddAdditionalAccounts>>,
    #[serde(rename = "masterAccountId", skip_serializing_if = "Option::is_none")]
    pub master_account_id: Option<String>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "inputLanguage", skip_serializing_if = "Option::is_none")]
    pub input_language: Option<InputLanguage>,
    #[serde(rename = "translation", skip_serializing_if = "Option::is_none")]
    pub translation: Option<bool>,
    #[serde(rename = "paperAccount", skip_serializing_if = "Option::is_none")]
    pub paper_account: Option<bool>,
}

impl Application {
    pub fn new() -> Application {
        Application {
            customer: None,
            accounts: None,
            users: None,
            documents: None,
            additional_accounts: None,
            master_account_id: None,
            id: None,
            input_language: None,
            translation: None,
            paper_account: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InputLanguage {
    #[serde(rename = "en")]
    En,
    #[serde(rename = "zh-Hans")]
    ZhHans,
    #[serde(rename = "ja")]
    Ja,
    #[serde(rename = "ru")]
    Ru,
}

impl Default for InputLanguage {
    fn default() -> InputLanguage {
        Self::En
    }
}

