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
pub struct AccountSupportType {
    #[serde(rename = "businessDescription", skip_serializing_if = "Option::is_none")]
    pub business_description: Option<String>,
    #[serde(rename = "primaryContributor", skip_serializing_if = "Option::is_none")]
    pub primary_contributor: Option<Box<models::PrimaryContributorType>>,
    #[serde(rename = "administrator", skip_serializing_if = "Option::is_none")]
    pub administrator: Option<Box<models::AdministratorType>>,
    #[serde(rename = "administratorContactPerson", skip_serializing_if = "Option::is_none")]
    pub administrator_contact_person: Option<Box<models::AdministratorContactPersonType>>,
    #[serde(rename = "isOwnersResideUS", skip_serializing_if = "Option::is_none")]
    pub is_owners_reside_us: Option<bool>,
    #[serde(rename = "isSolicitOwnersResideUS", skip_serializing_if = "Option::is_none")]
    pub is_solicit_owners_reside_us: Option<bool>,
    #[serde(rename = "isAcceptOwnersResideUS", skip_serializing_if = "Option::is_none")]
    pub is_accept_owners_reside_us: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl AccountSupportType {
    pub fn new() -> AccountSupportType {
        AccountSupportType {
            business_description: None,
            primary_contributor: None,
            administrator: None,
            administrator_contact_person: None,
            is_owners_reside_us: None,
            is_solicit_owners_reside_us: None,
            is_accept_owners_reside_us: None,
            r#type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "FINANCIALINSTITUTION")]
    Financialinstitution,
    #[serde(rename = "PROPRIETARYTRADING")]
    Proprietarytrading,
    #[serde(rename = "FAMILYINVVEHICLE")]
    Familyinvvehicle,
    #[serde(rename = "OPERATINGBUSINESS")]
    Operatingbusiness,
    #[serde(rename = "BROKERDEALER")]
    Brokerdealer,
    #[serde(rename = "LICENSEDADVISOR")]
    Licensedadvisor,
}

impl Default for Type {
    fn default() -> Type {
        Self::Financialinstitution
    }
}

