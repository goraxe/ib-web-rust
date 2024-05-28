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
pub struct DocumentSubmission {
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<models::Document>>,
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
    #[serde(rename = "inputLanguage", skip_serializing_if = "Option::is_none")]
    pub input_language: Option<InputLanguage>,
    #[serde(rename = "hasTranslation", skip_serializing_if = "Option::is_none")]
    pub has_translation: Option<bool>,
}

impl DocumentSubmission {
    pub fn new() -> DocumentSubmission {
        DocumentSubmission {
            documents: None,
            reference_account_id: None,
            input_language: None,
            has_translation: None,
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

