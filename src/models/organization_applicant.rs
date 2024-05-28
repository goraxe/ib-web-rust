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
pub struct OrganizationApplicant {
    #[serde(rename = "identifications", skip_serializing_if = "Option::is_none")]
    pub identifications: Option<Vec<models::OrganizationIdentification>>,
    #[serde(rename = "accountSupport", skip_serializing_if = "Option::is_none")]
    pub account_support: Option<Box<models::AccountSupportType>>,
    #[serde(rename = "financialInformation", skip_serializing_if = "Option::is_none")]
    pub financial_information: Option<Vec<models::FinancialInformation>>,
    #[serde(rename = "accreditedInvestorInformation", skip_serializing_if = "Option::is_none")]
    pub accredited_investor_information: Option<Box<models::AccreditedInvestorInformation>>,
    #[serde(rename = "regulatoryInformation", skip_serializing_if = "Option::is_none")]
    pub regulatory_information: Option<Vec<models::RegulatoryInformation>>,
    #[serde(rename = "managingOwner", skip_serializing_if = "Option::is_none")]
    pub managing_owner: Option<Box<models::ManagingOwner>>,
    #[serde(rename = "associatedEntities", skip_serializing_if = "Option::is_none")]
    pub associated_entities: Option<Box<models::AssociatedEntities>>,
    #[serde(rename = "regulatedMemberships", skip_serializing_if = "Option::is_none")]
    pub regulated_memberships: Option<Box<models::RegulatedMemberships>>,
    #[serde(rename = "taxResidencies", skip_serializing_if = "Option::is_none")]
    pub tax_residencies: Option<Vec<models::TaxResidency>>,
    #[serde(rename = "w8BenE", skip_serializing_if = "Option::is_none")]
    pub w8_ben_e: Option<Box<models::FormW8Bene>>,
    #[serde(rename = "w8IMY", skip_serializing_if = "Option::is_none")]
    pub w8_imy: Option<Box<models::FormW8Imy>>,
    #[serde(rename = "withholdingStatement", skip_serializing_if = "Option::is_none")]
    pub withholding_statement: Option<Box<models::WithholdingStatementType>>,
    #[serde(rename = "typeOfTrading", skip_serializing_if = "Option::is_none")]
    pub type_of_trading: Option<TypeOfTrading>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "isOrgUsSubsidiary", skip_serializing_if = "Option::is_none")]
    pub is_org_us_subsidiary: Option<bool>,
    #[serde(rename = "isQualifiedIntermediary", skip_serializing_if = "Option::is_none")]
    pub is_qualified_intermediary: Option<bool>,
    #[serde(rename = "hasAssumedPrimaryReporting", skip_serializing_if = "Option::is_none")]
    pub has_assumed_primary_reporting: Option<bool>,
    #[serde(rename = "hasAcceptedPrimaryWithholding", skip_serializing_if = "Option::is_none")]
    pub has_accepted_primary_withholding: Option<bool>,
    #[serde(rename = "usTaxPurposeType", skip_serializing_if = "Option::is_none")]
    pub us_tax_purpose_type: Option<UsTaxPurposeType>,
}

impl OrganizationApplicant {
    pub fn new() -> OrganizationApplicant {
        OrganizationApplicant {
            identifications: None,
            account_support: None,
            financial_information: None,
            accredited_investor_information: None,
            regulatory_information: None,
            managing_owner: None,
            associated_entities: None,
            regulated_memberships: None,
            tax_residencies: None,
            w8_ben_e: None,
            w8_imy: None,
            withholding_statement: None,
            type_of_trading: None,
            r#type: None,
            is_org_us_subsidiary: None,
            is_qualified_intermediary: None,
            has_assumed_primary_reporting: None,
            has_accepted_primary_withholding: None,
            us_tax_purpose_type: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TypeOfTrading {
    #[serde(rename = "FIRM")]
    Firm,
    #[serde(rename = "CUSTOMER")]
    Customer,
}

impl Default for TypeOfTrading {
    fn default() -> TypeOfTrading {
        Self::Firm
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "LLC")]
    Llc,
    #[serde(rename = "CORPORATION")]
    Corporation,
    #[serde(rename = "PARTNERSHIP")]
    Partnership,
    #[serde(rename = "UNINCORPORATED BUSINESS")]
    UnincorporatedBusiness,
}

impl Default for Type {
    fn default() -> Type {
        Self::Llc
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum UsTaxPurposeType {
    #[serde(rename = "C")]
    C,
    #[serde(rename = "P")]
    P,
    #[serde(rename = "E")]
    E,
}

impl Default for UsTaxPurposeType {
    fn default() -> UsTaxPurposeType {
        Self::C
    }
}
