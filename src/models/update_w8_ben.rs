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
pub struct UpdateW8Ben {
    #[serde(rename = "taxPayerDetails", skip_serializing_if = "Option::is_none")]
    pub tax_payer_details: Option<Box<models::TaxPayerDetails>>,
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<models::Document>>,
    #[serde(rename = "inputLanguage", skip_serializing_if = "Option::is_none")]
    pub input_language: Option<InputLanguage>,
    #[serde(rename = "translation", skip_serializing_if = "Option::is_none")]
    pub translation: Option<bool>,
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
}

impl UpdateW8Ben {
    pub fn new() -> UpdateW8Ben {
        UpdateW8Ben {
            tax_payer_details: None,
            documents: None,
            input_language: None,
            translation: None,
            reference_account_id: None,
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

