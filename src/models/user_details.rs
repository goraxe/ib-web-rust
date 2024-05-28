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
pub struct UserDetails {
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<Box<models::Name>>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "residence", skip_serializing_if = "Option::is_none")]
    pub residence: Option<Box<models::Residence>>,
    #[serde(rename = "mailingAddress", skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<Box<models::MailingAddress>>,
    #[serde(rename = "identification", skip_serializing_if = "Option::is_none")]
    pub identification: Option<Box<models::Identification>>,
    #[serde(rename = "taxResidencies", skip_serializing_if = "Option::is_none")]
    pub tax_residencies: Option<Vec<models::TaxResidency>>,
    #[serde(rename = "dateOfBirth", skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<Box<models::DateOfBirth>>,
    #[serde(rename = "sameMailAddress", skip_serializing_if = "Option::is_none")]
    pub same_mail_address: Option<bool>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
}

impl UserDetails {
    pub fn new() -> UserDetails {
        UserDetails {
            name: None,
            email: None,
            residence: None,
            mailing_address: None,
            identification: None,
            tax_residencies: None,
            date_of_birth: None,
            same_mail_address: None,
            external_id: None,
        }
    }
}

