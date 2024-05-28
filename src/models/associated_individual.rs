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
pub struct AssociatedIndividual {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<models::IndividualName>>,
    #[serde(rename = "nativeName", skip_serializing_if = "Option::is_none")]
    pub native_name: Option<Box<models::IndividualName>>,
    #[serde(rename = "birthName", skip_serializing_if = "Option::is_none")]
    pub birth_name: Option<Box<models::IndividualName>>,
    #[serde(rename = "motherMaidenName", skip_serializing_if = "Option::is_none")]
    pub mother_maiden_name: Option<Box<models::IndividualName>>,
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "countryOfBirth", skip_serializing_if = "Option::is_none")]
    pub country_of_birth: Option<String>,
    #[serde(rename = "cityOfBirth", skip_serializing_if = "Option::is_none")]
    pub city_of_birth: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<Gender>,
    #[serde(rename = "maritalStatus", skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<MaritalStatus>,
    #[serde(rename = "numDependents", skip_serializing_if = "Option::is_none")]
    pub num_dependents: Option<i32>,
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: Option<Box<models::Address>>,
    #[serde(rename = "mailingAddress", skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<Box<models::Address>>,
    #[serde(rename = "phones", skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<models::PhoneInfo>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "identification", skip_serializing_if = "Option::is_none")]
    pub identification: Option<Box<models::Identification>>,
    #[serde(rename = "employmentType", skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<String>,
    #[serde(rename = "employmentDetails", skip_serializing_if = "Option::is_none")]
    pub employment_details: Option<Box<models::EmploymentDetails>>,
    #[serde(rename = "employeeTitle", skip_serializing_if = "Option::is_none")]
    pub employee_title: Option<String>,
    #[serde(rename = "taxResidencies", skip_serializing_if = "Option::is_none")]
    pub tax_residencies: Option<Vec<models::TaxResidency>>,
    #[serde(rename = "w9", skip_serializing_if = "Option::is_none")]
    pub w9: Option<Box<models::FormW9>>,
    #[serde(rename = "w8Ben", skip_serializing_if = "Option::is_none")]
    pub w8_ben: Option<Box<models::FormW8Ben>>,
    #[serde(rename = "crs", skip_serializing_if = "Option::is_none")]
    pub crs: Option<Box<models::FormCrs>>,
    #[serde(rename = "prohibitedCountryQuestionnaire", skip_serializing_if = "Option::is_none")]
    pub prohibited_country_questionnaire: Option<Box<models::ProhibitedCountryQuestionnaireList>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "userId", skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "hasSameMailAddress", skip_serializing_if = "Option::is_none")]
    pub has_same_mail_address: Option<bool>,
    #[serde(rename = "isAuthorizedToSignOnBehalfOfOwner", skip_serializing_if = "Option::is_none")]
    pub is_authorized_to_sign_on_behalf_of_owner: Option<bool>,
    #[serde(rename = "isAuthorizedTrader", skip_serializing_if = "Option::is_none")]
    pub is_authorized_trader: Option<bool>,
    #[serde(rename = "isUsTaxResident", skip_serializing_if = "Option::is_none")]
    pub is_us_tax_resident: Option<bool>,
    #[serde(rename = "isTranslated", skip_serializing_if = "Option::is_none")]
    pub is_translated: Option<bool>,
    #[serde(rename = "isPrimaryTrustee", skip_serializing_if = "Option::is_none")]
    pub is_primary_trustee: Option<bool>,
    #[serde(rename = "ownership", skip_serializing_if = "Option::is_none")]
    pub ownership: Option<Box<models::Ownership>>,
    #[serde(rename = "titles", skip_serializing_if = "Option::is_none")]
    pub titles: Option<Vec<models::Title>>,
    #[serde(rename = "isAuthorizedPerson", skip_serializing_if = "Option::is_none")]
    pub is_authorized_person: Option<bool>,
    #[serde(rename = "referenceUsername", skip_serializing_if = "Option::is_none")]
    pub reference_username: Option<String>,
}

impl AssociatedIndividual {
    pub fn new() -> AssociatedIndividual {
        AssociatedIndividual {
            name: None,
            native_name: None,
            birth_name: None,
            mother_maiden_name: None,
            date_of_birth: None,
            country_of_birth: None,
            city_of_birth: None,
            gender: None,
            marital_status: None,
            num_dependents: None,
            residence: None,
            mailing_address: None,
            phones: None,
            email: None,
            identification: None,
            employment_type: None,
            employment_details: None,
            employee_title: None,
            tax_residencies: None,
            w9: None,
            w8_ben: None,
            crs: None,
            prohibited_country_questionnaire: None,
            id: None,
            external_id: None,
            user_id: None,
            has_same_mail_address: None,
            is_authorized_to_sign_on_behalf_of_owner: None,
            is_authorized_trader: None,
            is_us_tax_resident: None,
            is_translated: None,
            is_primary_trustee: None,
            ownership: None,
            titles: None,
            is_authorized_person: None,
            reference_username: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Gender {
    #[serde(rename = "M")]
    M,
    #[serde(rename = "F")]
    F,
    #[serde(rename = "Male")]
    Male,
    #[serde(rename = "Female")]
    Female,
}

impl Default for Gender {
    fn default() -> Gender {
        Self::M
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum MaritalStatus {
    #[serde(rename = "S")]
    S,
    #[serde(rename = "M")]
    M,
    #[serde(rename = "W")]
    W,
    #[serde(rename = "D")]
    D,
    #[serde(rename = "C")]
    C,
}

impl Default for MaritalStatus {
    fn default() -> MaritalStatus {
        Self::S
    }
}

