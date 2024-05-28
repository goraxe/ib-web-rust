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
pub struct Title {
    #[serde(rename = "value", skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
}

impl Title {
    pub fn new() -> Title {
        Title {
            value: None,
            code: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "Account Holder")]
    AccountHolder,
    #[serde(rename = "FIRST HOLDER")]
    FirstHolder,
    #[serde(rename = "SECOND HOLDER")]
    SecondHolder,
    #[serde(rename = "TRADER")]
    Trader,
    #[serde(rename = "CEO")]
    Ceo,
    #[serde(rename = "SECRETARY")]
    Secretary,
    #[serde(rename = "TREASURER")]
    Treasurer,
    #[serde(rename = "OWNER")]
    Owner,
    #[serde(rename = "PRINCIPAL")]
    Principal,
    #[serde(rename = "SHAREHOLDER")]
    Shareholder,
    #[serde(rename = "TRUSTEE")]
    Trustee,
    #[serde(rename = "BENEFICIARY")]
    Beneficiary,
    #[serde(rename = "GRANTOR")]
    Grantor,
    #[serde(rename = "Employee")]
    Employee,
    #[serde(rename = "CONTINGENT")]
    Contingent,
    #[serde(rename = "IRA_BENEFICIARY")]
    IraBeneficiary,
    #[serde(rename = "IRA DECEDENT")]
    IraDecedent,
    #[serde(rename = "COMP_OFFICER")]
    CompOfficer,
    #[serde(rename = "Other Officer")]
    OtherOfficer,
    #[serde(rename = "Controlling Officer")]
    ControllingOfficer,
    #[serde(rename = "SIGNATORY")]
    Signatory,
    #[serde(rename = "NON-EMPLOYEE")]
    NonEmployee,
    #[serde(rename = "CUSTODIAN")]
    Custodian,
    #[serde(rename = "SUCCESSOR_CUSTODIAN")]
    SuccessorCustodian,
    #[serde(rename = "DIRECTOR")]
    Director,
    #[serde(rename = "PARTNER")]
    Partner,
    #[serde(rename = "CUSTODIAN EMPLOYEE")]
    CustodianEmployee,
    #[serde(rename = "SUCCESSOR CUSTODIAN EMPLOYEE")]
    SuccessorCustodianEmployee,
    #[serde(rename = "SPOUSE")]
    Spouse,
    #[serde(rename = "Successor Holder")]
    SuccessorHolder,
}

impl Default for Code {
    fn default() -> Code {
        Self::AccountHolder
    }
}

