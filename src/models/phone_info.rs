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
pub struct PhoneInfo {
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "number", skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    #[serde(rename = "country", skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "isVerified", skip_serializing_if = "Option::is_none")]
    pub is_verified: Option<bool>,
}

impl PhoneInfo {
    pub fn new() -> PhoneInfo {
        PhoneInfo {
            r#type: None,
            number: None,
            country: None,
            is_verified: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "Work")]
    Work,
    #[serde(rename = "Home")]
    Home,
    #[serde(rename = "Fax")]
    Fax,
    #[serde(rename = "Mobile")]
    Mobile,
    #[serde(rename = "Mobile (work)")]
    MobileLeftParenthesisWorkRightParenthesis,
    #[serde(rename = "Mobile (other)")]
    MobileLeftParenthesisOtherRightParenthesis,
    #[serde(rename = "Business")]
    Business,
    #[serde(rename = "Other (voice)")]
    OtherLeftParenthesisVoiceRightParenthesis,
}

impl Default for Type {
    fn default() -> Type {
        Self::Work
    }
}

