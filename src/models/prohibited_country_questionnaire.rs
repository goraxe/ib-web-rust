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
pub struct ProhibitedCountryQuestionnaire {
    #[serde(rename = "prohibitedQuestionnaireDetails", skip_serializing_if = "Option::is_none")]
    pub prohibited_questionnaire_details: Option<Vec<models::ProhibitedQuestionnaireDetail>>,
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "entityId", skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
}

impl ProhibitedCountryQuestionnaire {
    pub fn new() -> ProhibitedCountryQuestionnaire {
        ProhibitedCountryQuestionnaire {
            prohibited_questionnaire_details: None,
            reference_account_id: None,
            external_id: None,
            entity_id: None,
        }
    }
}
