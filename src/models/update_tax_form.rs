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
pub struct UpdateTaxForm {
    #[serde(rename = "localTaxForms", skip_serializing_if = "Option::is_none")]
    pub local_tax_forms: Option<Vec<models::LocalTaxForm>>,
    #[serde(rename = "w8Ben", skip_serializing_if = "Option::is_none")]
    pub w8_ben: Option<Box<models::FormW8Ben>>,
    #[serde(rename = "w8BenE", skip_serializing_if = "Option::is_none")]
    pub w8_ben_e: Option<Box<models::FormW8Bene>>,
    #[serde(rename = "w9", skip_serializing_if = "Option::is_none")]
    pub w9: Option<Box<models::FormW9>>,
    #[serde(rename = "hasTranslation", skip_serializing_if = "Option::is_none")]
    pub has_translation: Option<bool>,
    #[serde(rename = "inputLanguage", skip_serializing_if = "Option::is_none")]
    pub input_language: Option<InputLanguage>,
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
    #[serde(rename = "documents", skip_serializing_if = "Option::is_none")]
    pub documents: Option<Vec<models::Document>>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
}

impl UpdateTaxForm {
    pub fn new() -> UpdateTaxForm {
        UpdateTaxForm {
            local_tax_forms: None,
            w8_ben: None,
            w8_ben_e: None,
            w9: None,
            has_translation: None,
            input_language: None,
            reference_account_id: None,
            documents: None,
            user_name: None,
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
