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
pub struct AccountDetailsResponse {
    #[serde(rename = "error", skip_serializing_if = "Option::is_none")]
    pub error: Option<Box<models::ErrorResponse>>,
    #[serde(rename = "hasError", skip_serializing_if = "Option::is_none")]
    pub has_error: Option<bool>,
    #[serde(rename = "errorDescription", skip_serializing_if = "Option::is_none")]
    pub error_description: Option<String>,
    #[serde(rename = "account", skip_serializing_if = "Option::is_none")]
    pub account: Option<Box<models::AccountData>>,
    #[serde(rename = "associatedPersons", skip_serializing_if = "Option::is_none")]
    pub associated_persons: Option<Vec<models::AssociatedPerson>>,
    #[serde(rename = "associatedEntities", skip_serializing_if = "Option::is_none")]
    pub associated_entities: Option<Vec<models::AssociatedEntity>>,
    #[serde(rename = "withHoldingStatement", skip_serializing_if = "Option::is_none")]
    pub with_holding_statement: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "marketData", skip_serializing_if = "Option::is_none")]
    pub market_data: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "financialInformation", skip_serializing_if = "Option::is_none")]
    pub financial_information: Option<std::collections::HashMap<String, serde_json::Value>>,
    #[serde(rename = "sourcesOfWealth", skip_serializing_if = "Option::is_none")]
    pub sources_of_wealth: Option<Vec<std::collections::HashMap<String, serde_json::Value>>>,
    #[serde(rename = "tradeBundles", skip_serializing_if = "Option::is_none")]
    pub trade_bundles: Option<Vec<String>>,
    #[serde(rename = "individualIRABeneficiaries", skip_serializing_if = "Option::is_none")]
    pub individual_ira_beneficiaries: Option<Vec<models::IndividualIraBene>>,
    #[serde(rename = "entityIRABeneficiaries", skip_serializing_if = "Option::is_none")]
    pub entity_ira_beneficiaries: Option<Vec<models::EntityIraBene>>,
    #[serde(rename = "decedents", skip_serializing_if = "Option::is_none")]
    pub decedents: Option<Vec<std::collections::HashMap<String, String>>>,
    #[serde(rename = "restrictions", skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<Vec<models::RestrictionInfo>>,
}

impl AccountDetailsResponse {
    pub fn new() -> AccountDetailsResponse {
        AccountDetailsResponse {
            error: None,
            has_error: None,
            error_description: None,
            account: None,
            associated_persons: None,
            associated_entities: None,
            with_holding_statement: None,
            market_data: None,
            financial_information: None,
            sources_of_wealth: None,
            trade_bundles: None,
            individual_ira_beneficiaries: None,
            entity_ira_beneficiaries: None,
            decedents: None,
            restrictions: None,
        }
    }
}
