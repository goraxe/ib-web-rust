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
pub struct Identification {
    #[serde(rename = "citizenship", skip_serializing_if = "Option::is_none")]
    pub citizenship: Option<String>,
    #[serde(rename = "citizenship2", skip_serializing_if = "Option::is_none")]
    pub citizenship2: Option<String>,
    #[serde(rename = "citizenship3", skip_serializing_if = "Option::is_none")]
    pub citizenship3: Option<String>,
    #[serde(rename = "ssn", skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    #[serde(rename = "sin", skip_serializing_if = "Option::is_none")]
    pub sin: Option<String>,
    #[serde(rename = "driversLicense", skip_serializing_if = "Option::is_none")]
    pub drivers_license: Option<String>,
    #[serde(rename = "passport", skip_serializing_if = "Option::is_none")]
    pub passport: Option<String>,
    #[serde(rename = "alienCard", skip_serializing_if = "Option::is_none")]
    pub alien_card: Option<String>,
    #[serde(rename = "medicareCard", skip_serializing_if = "Option::is_none")]
    pub medicare_card: Option<String>,
    #[serde(rename = "cardColor", skip_serializing_if = "Option::is_none")]
    pub card_color: Option<CardColor>,
    #[serde(rename = "medicareReference", skip_serializing_if = "Option::is_none")]
    pub medicare_reference: Option<String>,
    #[serde(rename = "nationalCard", skip_serializing_if = "Option::is_none")]
    pub national_card: Option<String>,
    #[serde(rename = "issuingCountry", skip_serializing_if = "Option::is_none")]
    pub issuing_country: Option<String>,
    #[serde(rename = "issuingState", skip_serializing_if = "Option::is_none")]
    pub issuing_state: Option<String>,
    #[serde(rename = "rta", skip_serializing_if = "Option::is_none")]
    pub rta: Option<String>,
    #[serde(rename = "legalResidenceCountry", skip_serializing_if = "Option::is_none")]
    pub legal_residence_country: Option<String>,
    #[serde(rename = "legalResidenceState", skip_serializing_if = "Option::is_none")]
    pub legal_residence_state: Option<String>,
    #[serde(rename = "educationalQualification", skip_serializing_if = "Option::is_none")]
    pub educational_qualification: Option<String>,
    #[serde(rename = "fathersName", skip_serializing_if = "Option::is_none")]
    pub fathers_name: Option<String>,
    #[serde(rename = "hasGreenCard", skip_serializing_if = "Option::is_none")]
    pub has_green_card: Option<bool>,
    #[serde(rename = "panNumber", skip_serializing_if = "Option::is_none")]
    pub pan_number: Option<String>,
    #[serde(rename = "taxId", skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<String>,
    #[serde(rename = "proofOfAgeCard", skip_serializing_if = "Option::is_none")]
    pub proof_of_age_card: Option<String>,
    #[serde(rename = "hasExpirationDate", skip_serializing_if = "Option::is_none")]
    pub has_expiration_date: Option<bool>,
    #[serde(rename = "expirationDate", skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
}

impl Identification {
    pub fn new() -> Identification {
        Identification {
            citizenship: None,
            citizenship2: None,
            citizenship3: None,
            ssn: None,
            sin: None,
            drivers_license: None,
            passport: None,
            alien_card: None,
            medicare_card: None,
            card_color: None,
            medicare_reference: None,
            national_card: None,
            issuing_country: None,
            issuing_state: None,
            rta: None,
            legal_residence_country: None,
            legal_residence_state: None,
            educational_qualification: None,
            fathers_name: None,
            has_green_card: None,
            pan_number: None,
            tax_id: None,
            proof_of_age_card: None,
            has_expiration_date: None,
            expiration_date: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum CardColor {
    #[serde(rename = "BLUE")]
    Blue,
    #[serde(rename = "GREEN")]
    Green,
    #[serde(rename = "YELLOW")]
    Yellow,
}

impl Default for CardColor {
    fn default() -> CardColor {
        Self::Blue
    }
}
