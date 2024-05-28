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
pub struct PrimaryContributorType {
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "middleInitial", skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<Suffix>,
    #[serde(rename = "employer", skip_serializing_if = "Option::is_none")]
    pub employer: Option<String>,
    #[serde(rename = "occupation", skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    #[serde(rename = "address", skip_serializing_if = "Option::is_none")]
    pub address: Option<Box<models::Address>>,
    #[serde(rename = "sourceOfFunds", skip_serializing_if = "Option::is_none")]
    pub source_of_funds: Option<String>,
}

impl PrimaryContributorType {
    pub fn new() -> PrimaryContributorType {
        PrimaryContributorType {
            first_name: None,
            middle_initial: None,
            last_name: None,
            suffix: None,
            employer: None,
            occupation: None,
            address: None,
            source_of_funds: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Suffix {
    #[serde(rename = "Jr.")]
    JrPeriod,
    #[serde(rename = "Sr.")]
    SrPeriod,
    #[serde(rename = "I")]
    I,
    #[serde(rename = "II")]
    Ii,
    #[serde(rename = "III")]
    Iii,
    #[serde(rename = "IV")]
    Iv,
    #[serde(rename = "V")]
    V,
}

impl Default for Suffix {
    fn default() -> Suffix {
        Self::JrPeriod
    }
}
