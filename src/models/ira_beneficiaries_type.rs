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
pub struct IraBeneficiariesType {
    #[serde(rename = "primaryBeneficiaries", skip_serializing_if = "Option::is_none")]
    pub primary_beneficiaries: Option<Vec<models::IraPrimaryBeneficiary>>,
    #[serde(rename = "primaryBeneficiaryEntities", skip_serializing_if = "Option::is_none")]
    pub primary_beneficiary_entities: Option<Vec<models::IraPrimaryBeneficiaryEntity>>,
    #[serde(rename = "contingentBeneficiaries", skip_serializing_if = "Option::is_none")]
    pub contingent_beneficiaries: Option<Vec<models::IraContingentBeneficiary>>,
    #[serde(rename = "contingentBeneficiaryEntities", skip_serializing_if = "Option::is_none")]
    pub contingent_beneficiary_entities: Option<Vec<models::IraContingentBeneficiaryEntity>>,
    #[serde(rename = "isSpousePrimaryBeneficary", skip_serializing_if = "Option::is_none")]
    pub is_spouse_primary_beneficary: Option<bool>,
    #[serde(rename = "isSuccessor", skip_serializing_if = "Option::is_none")]
    pub is_successor: Option<bool>,
}

impl IraBeneficiariesType {
    pub fn new() -> IraBeneficiariesType {
        IraBeneficiariesType {
            primary_beneficiaries: None,
            primary_beneficiary_entities: None,
            contingent_beneficiaries: None,
            contingent_beneficiary_entities: None,
            is_spouse_primary_beneficary: None,
            is_successor: None,
        }
    }
}

