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
pub struct AssociatedPerson {
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<i32>,
    #[serde(rename = "externalCode", skip_serializing_if = "Option::is_none")]
    pub external_code: Option<String>,
    #[serde(rename = "firstName", skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName", skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(rename = "middleInitial", skip_serializing_if = "Option::is_none")]
    pub middle_initial: Option<String>,
    #[serde(rename = "lastName", skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "suffix", skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(rename = "passwordDate", skip_serializing_if = "Option::is_none")]
    pub password_date: Option<String>,
    #[serde(rename = "userStatus", skip_serializing_if = "Option::is_none")]
    pub user_status: Option<String>,
    #[serde(rename = "lastLogin", skip_serializing_if = "Option::is_none")]
    pub last_login: Option<String>,
    #[serde(rename = "gender", skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(rename = "maritalStatus", skip_serializing_if = "Option::is_none")]
    pub marital_status: Option<String>,
    #[serde(rename = "salutation", skip_serializing_if = "Option::is_none")]
    pub salutation: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "countryOfCitizenship", skip_serializing_if = "Option::is_none")]
    pub country_of_citizenship: Option<String>,
    #[serde(rename = "countryOfBirth", skip_serializing_if = "Option::is_none")]
    pub country_of_birth: Option<String>,
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    #[serde(rename = "motersMaidenName", skip_serializing_if = "Option::is_none")]
    pub moters_maiden_name: Option<String>,
    #[serde(rename = "numberOfDependents", skip_serializing_if = "Option::is_none")]
    pub number_of_dependents: Option<i32>,
    #[serde(rename = "securityDevice", skip_serializing_if = "Option::is_none")]
    pub security_device: Option<String>,
    #[serde(rename = "commercial", skip_serializing_if = "Option::is_none")]
    pub commercial: Option<String>,
    #[serde(rename = "phones", skip_serializing_if = "Option::is_none")]
    pub phones: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "mailing", skip_serializing_if = "Option::is_none")]
    pub mailing: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "associations", skip_serializing_if = "Option::is_none")]
    pub associations: Option<Vec<String>>,
    #[serde(rename = "identityDocuments", skip_serializing_if = "Option::is_none")]
    pub identity_documents: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "employmentType", skip_serializing_if = "Option::is_none")]
    pub employment_type: Option<String>,
    #[serde(rename = "employmentDetails", skip_serializing_if = "Option::is_none")]
    pub employment_details: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "subscribedServices", skip_serializing_if = "Option::is_none")]
    pub subscribed_services: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "taxTreatyDetails", skip_serializing_if = "Option::is_none")]
    pub tax_treaty_details: Option<Vec<std::collections::HashMap<String, String>>>,
}

impl AssociatedPerson {
    pub fn new() -> AssociatedPerson {
        AssociatedPerson {
            entity_id: None,
            external_code: None,
            first_name: None,
            middle_name: None,
            middle_initial: None,
            last_name: None,
            suffix: None,
            username: None,
            password_date: None,
            user_status: None,
            last_login: None,
            gender: None,
            marital_status: None,
            salutation: None,
            email: None,
            country_of_citizenship: None,
            country_of_birth: None,
            date_of_birth: None,
            moters_maiden_name: None,
            number_of_dependents: None,
            security_device: None,
            commercial: None,
            phones: None,
            residence: None,
            mailing: None,
            associations: None,
            identity_documents: None,
            employment_type: None,
            employment_details: None,
            subscribed_services: None,
            tax_treaty_details: None,
        }
    }
}

