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
pub struct AccreditedInvestor {
    #[serde(rename = "qualifiedPurchaser", skip_serializing_if = "Option::is_none")]
    pub qualified_purchaser: Option<Box<models::QualifiedPurchaser>>,
    #[serde(rename = "eligibleContractParticipant", skip_serializing_if = "Option::is_none")]
    pub eligible_contract_participant: Option<Box<models::EligibleContractParticipant>>,
    #[serde(rename = "signedBy", skip_serializing_if = "Option::is_none")]
    pub signed_by: Option<Vec<String>>,
    #[serde(rename = "referenceAccountId", skip_serializing_if = "Option::is_none")]
    pub reference_account_id: Option<String>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(rename = "signature", skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
}

impl AccreditedInvestor {
    pub fn new() -> AccreditedInvestor {
        AccreditedInvestor {
            qualified_purchaser: None,
            eligible_contract_participant: None,
            signed_by: None,
            reference_account_id: None,
            status: None,
            signature: None,
        }
    }
}
