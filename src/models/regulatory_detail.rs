/*
 * IB REST API
 *
 * The IB REST API reference documentation
 *
 * The version of the OpenAPI document: 2.9.0
 * Contact: api@interactivebrokers.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct RegulatoryDetail {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<bool>,
    #[serde(rename = "details", skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "detail", skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(rename = "externalIndividualId", skip_serializing_if = "Option::is_none")]
    pub external_individual_id: Option<String>,
}

impl RegulatoryDetail {
    pub fn new() -> RegulatoryDetail {
        RegulatoryDetail {
            code: None,
            status: None,
            details: None,
            detail: None,
            external_individual_id: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "CRIMINAL")]
    Criminal,
    #[serde(rename = "AFFILIATION")]
    Affiliation,
    #[serde(rename = "CFTCREGISTERED")]
    Cftcregistered,
    #[serde(rename = "IBACCOUNTS")]
    Ibaccounts,
    #[serde(rename = "REGULATORYCONTROL")]
    Regulatorycontrol,
    #[serde(rename = "EmployeePubTrade")]
    EmployeePubTrade,
    #[serde(rename = "ControlPubTraded")]
    ControlPubTraded,
    #[serde(rename = "BROKERDEALER")]
    Brokerdealer,
    #[serde(rename = "EXCHANGEMEMBERSHIP")]
    Exchangemembership,
    #[serde(rename = "STOCKCONTROL")]
    Stockcontrol,
    #[serde(rename = "DISPUTE")]
    Dispute,
    #[serde(rename = "INVESTIGATION")]
    Investigation,
    #[serde(rename = "MEMBERSHIP")]
    Membership,
    #[serde(rename = "AUSEXPOSURE")]
    Ausexposure,
    #[serde(rename = "CONTROLLER")]
    Controller,
    #[serde(rename = "POLITICALMILITARYDIPLOMATIC")]
    Politicalmilitarydiplomatic,
    #[serde(rename = "FOREIGN_BANK")]
    ForeignBank,
    #[serde(rename = "BROKER_DEALER")]
    BrokerDealer,
    #[serde(rename = "FUTURES_COMMISSION_MERCHANT")]
    FuturesCommissionMerchant,
    #[serde(rename = "MUTUAL_FUND")]
    MutualFund,
    #[serde(rename = "FOREIGN_EXCHANGE")]
    ForeignExchange,
    #[serde(rename = "MONEY_TRANSMITTER")]
    MoneyTransmitter,
    #[serde(rename = "EMPLOYEE_BENEFIT_PLAN")]
    EmployeeBenefitPlan,
    #[serde(rename = "US_BANK")]
    UsBank,
    #[serde(rename = "US_SWAP_DEALER")]
    UsSwapDealer,
    #[serde(rename = "US_SWAP_PARTICIPANT")]
    UsSwapParticipant,
    #[serde(rename = "US_INSURANCE_COMPANY")]
    UsInsuranceCompany,
    #[serde(rename = "NON_US_INSURANCE_COMPANY")]
    NonUsInsuranceCompany,
    #[serde(rename = "US_DEPT")]
    UsDept,
    #[serde(rename = "FINANCIAL_ADVISOR")]
    FinancialAdvisor,
    #[serde(rename = "HIGH_RISK_CONTRIBUTION")]
    HighRiskContribution,
}

impl Default for Code {
    fn default() -> Code {
        Self::Criminal
    }
}

