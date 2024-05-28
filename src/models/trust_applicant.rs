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
pub struct TrustApplicant {
    #[serde(rename = "identification", skip_serializing_if = "Option::is_none")]
    pub identification: Option<Vec<models::TrustIdentification>>,
    #[serde(rename = "financialInformation", skip_serializing_if = "Option::is_none")]
    pub financial_information: Option<Vec<models::FinancialInformation>>,
    #[serde(rename = "regulatoryInformation", skip_serializing_if = "Option::is_none")]
    pub regulatory_information: Option<Vec<models::RegulatoryInformation>>,
    #[serde(rename = "regulatedMemberships", skip_serializing_if = "Option::is_none")]
    pub regulated_memberships: Option<Box<models::RegulatedMemberships>>,
    #[serde(rename = "accreditedInvestorInformation", skip_serializing_if = "Option::is_none")]
    pub accredited_investor_information: Option<Box<models::AccreditedInvestorInformation>>,
    #[serde(rename = "trustees", skip_serializing_if = "Option::is_none")]
    pub trustees: Option<Box<models::TrusteesType>>,
    #[serde(rename = "beneficiaries", skip_serializing_if = "Option::is_none")]
    pub beneficiaries: Option<Box<models::AssociationTypeEntities>>,
    #[serde(rename = "grantors", skip_serializing_if = "Option::is_none")]
    pub grantors: Option<Box<models::AssociationTypeEntities>>,
    #[serde(rename = "taxResidencies", skip_serializing_if = "Option::is_none")]
    pub tax_residencies: Option<Vec<models::TaxResidency>>,
    #[serde(rename = "w8BenE", skip_serializing_if = "Option::is_none")]
    pub w8_ben_e: Option<Box<models::FormW8Bene>>,
    #[serde(rename = "w8IMY", skip_serializing_if = "Option::is_none")]
    pub w8_imy: Option<Box<models::FormW8Imy>>,
    #[serde(rename = "withholdingStatement", skip_serializing_if = "Option::is_none")]
    pub withholding_statement: Option<Box<models::WithholdingStatementType>>,
    #[serde(rename = "thirdPartyManagement", skip_serializing_if = "Option::is_none")]
    pub third_party_management: Option<bool>,
    #[serde(rename = "trustType", skip_serializing_if = "Option::is_none")]
    pub trust_type: Option<TrustType>,
}

impl TrustApplicant {
    pub fn new() -> TrustApplicant {
        TrustApplicant {
            identification: None,
            financial_information: None,
            regulatory_information: None,
            regulated_memberships: None,
            accredited_investor_information: None,
            trustees: None,
            beneficiaries: None,
            grantors: None,
            tax_residencies: None,
            w8_ben_e: None,
            w8_imy: None,
            withholding_statement: None,
            third_party_management: None,
            trust_type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TrustType {
    #[serde(rename = "COMPLEX_TRUST")]
    ComplexTrust,
    #[serde(rename = "SINGLE_TRUST")]
    SingleTrust,
    #[serde(rename = "GRANTOR_TRUST")]
    GrantorTrust,
    #[serde(rename = "US_TAXABLE_TRUST")]
    UsTaxableTrust,
}

impl Default for TrustType {
    fn default() -> TrustType {
        Self::ComplexTrust
    }
}
