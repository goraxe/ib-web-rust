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
pub struct Customer {
    #[serde(rename = "organization", skip_serializing_if = "Option::is_none")]
    pub organization: Option<Box<models::OrganizationApplicant>>,
    #[serde(rename = "accountHolder", skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<Box<models::IndividualApplicant>>,
    #[serde(rename = "jointHolders", skip_serializing_if = "Option::is_none")]
    pub joint_holders: Option<Box<models::JointApplicant>>,
    #[serde(rename = "trust", skip_serializing_if = "Option::is_none")]
    pub trust: Option<Box<models::TrustApplicant>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "isTransferUsMicroCapStock", skip_serializing_if = "Option::is_none")]
    pub is_transfer_us_micro_cap_stock: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
    #[serde(rename = "prefix", skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(rename = "userName", skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "userNameAlias", skip_serializing_if = "Option::is_none")]
    pub user_name_alias: Option<String>,
    #[serde(rename = "userNameSource", skip_serializing_if = "Option::is_none")]
    pub user_name_source: Option<String>,
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(rename = "isMdStatusNonPro", skip_serializing_if = "Option::is_none")]
    pub is_md_status_non_pro: Option<bool>,
    #[serde(rename = "preferredPrimaryLanguage", skip_serializing_if = "Option::is_none")]
    pub preferred_primary_language: Option<String>,
    #[serde(rename = "preferredSecondaryLanguage", skip_serializing_if = "Option::is_none")]
    pub preferred_secondary_language: Option<String>,
    #[serde(rename = "legalResidenceCountry", skip_serializing_if = "Option::is_none")]
    pub legal_residence_country: Option<String>,
    #[serde(rename = "taxTreatyCountry", skip_serializing_if = "Option::is_none")]
    pub tax_treaty_country: Option<String>,
    #[serde(rename = "meetAmlStandard", skip_serializing_if = "Option::is_none")]
    pub meet_aml_standard: Option<String>,
    #[serde(rename = "meetsAmlStandard", skip_serializing_if = "Option::is_none")]
    pub meets_aml_standard: Option<String>,
    #[serde(rename = "hasDirectTradingAccess", skip_serializing_if = "Option::is_none")]
    pub has_direct_trading_access: Option<bool>,
    #[serde(rename = "originCountry", skip_serializing_if = "Option::is_none")]
    pub origin_country: Option<String>,
    #[serde(rename = "terminationAge", skip_serializing_if = "Option::is_none")]
    pub termination_age: Option<i32>,
    #[serde(rename = "governingState", skip_serializing_if = "Option::is_none")]
    pub governing_state: Option<String>,
    #[serde(rename = "isOptForDebitCard", skip_serializing_if = "Option::is_none")]
    pub is_opt_for_debit_card: Option<bool>,
    #[serde(rename = "isRoboFaClient", skip_serializing_if = "Option::is_none")]
    pub is_robo_fa_client: Option<bool>,
    #[serde(rename = "isIndependentAccount", skip_serializing_if = "Option::is_none")]
    pub is_independent_account: Option<bool>,
    #[serde(rename = "isPaperAccount", skip_serializing_if = "Option::is_none")]
    pub is_paper_account: Option<bool>,
}

impl Customer {
    pub fn new() -> Customer {
        Customer {
            organization: None,
            account_holder: None,
            joint_holders: None,
            trust: None,
            id: None,
            external_id: None,
            is_transfer_us_micro_cap_stock: None,
            r#type: None,
            prefix: None,
            user_name: None,
            user_name_alias: None,
            user_name_source: None,
            email: None,
            is_md_status_non_pro: None,
            preferred_primary_language: None,
            preferred_secondary_language: None,
            legal_residence_country: None,
            tax_treaty_country: None,
            meet_aml_standard: None,
            meets_aml_standard: None,
            has_direct_trading_access: None,
            origin_country: None,
            termination_age: None,
            governing_state: None,
            is_opt_for_debit_card: None,
            is_robo_fa_client: None,
            is_independent_account: None,
            is_paper_account: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "INDIVIDUAL")]
    Individual,
    #[serde(rename = "JOINT")]
    Joint,
    #[serde(rename = "TRUST")]
    Trust,
    #[serde(rename = "UGMA")]
    Ugma,
    #[serde(rename = "UTMA")]
    Utma,
    #[serde(rename = "ORG")]
    Org,
    #[serde(rename = "IRA")]
    Ira,
}

impl Default for Type {
    fn default() -> Type {
        Self::Individual
    }
}

