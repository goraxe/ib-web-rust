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
pub struct FinancialInformation {
    #[serde(rename = "investmentExperience", skip_serializing_if = "Option::is_none")]
    pub investment_experience: Option<Vec<models::AssetExperience>>,
    #[serde(rename = "investmentObjectives", skip_serializing_if = "Option::is_none")]
    pub investment_objectives: Option<Vec<InvestmentObjectives>>,
    #[serde(rename = "additionalSourcesOfIncome", skip_serializing_if = "Option::is_none")]
    pub additional_sources_of_income: Option<Vec<models::SourceOfIncomeType>>,
    #[serde(rename = "sourcesOfWealth", skip_serializing_if = "Option::is_none")]
    pub sources_of_wealth: Option<Vec<models::SourceOfWealthType>>,
    #[serde(rename = "soiQuestionnaire", skip_serializing_if = "Option::is_none")]
    pub soi_questionnaire: Option<Box<models::SoiQuestionnaire>>,
    #[serde(rename = "questionnaires", skip_serializing_if = "Option::is_none")]
    pub questionnaires: Option<Vec<models::QuestionnaireType>>,
    #[serde(rename = "netWorth", skip_serializing_if = "Option::is_none")]
    pub net_worth: Option<f64>,
    #[serde(rename = "liquidNetWorth", skip_serializing_if = "Option::is_none")]
    pub liquid_net_worth: Option<f64>,
    #[serde(rename = "annualNetIncome", skip_serializing_if = "Option::is_none")]
    pub annual_net_income: Option<f64>,
    #[serde(rename = "totalAssets", skip_serializing_if = "Option::is_none")]
    pub total_assets: Option<f64>,
    #[serde(rename = "sourceOfFunds", skip_serializing_if = "Option::is_none")]
    pub source_of_funds: Option<String>,
    #[serde(rename = "isTranslated", skip_serializing_if = "Option::is_none")]
    pub is_translated: Option<bool>,
}

impl FinancialInformation {
    pub fn new() -> FinancialInformation {
        FinancialInformation {
            investment_experience: None,
            investment_objectives: None,
            additional_sources_of_income: None,
            sources_of_wealth: None,
            soi_questionnaire: None,
            questionnaires: None,
            net_worth: None,
            liquid_net_worth: None,
            annual_net_income: None,
            total_assets: None,
            source_of_funds: None,
            is_translated: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvestmentObjectives {
    #[serde(rename = "Trading")]
    Trading,
    #[serde(rename = "Growth")]
    Growth,
    #[serde(rename = "Speculation")]
    Speculation,
    #[serde(rename = "Hedging")]
    Hedging,
    #[serde(rename = "Preservation")]
    Preservation,
    #[serde(rename = "Income")]
    Income,
}

impl Default for InvestmentObjectives {
    fn default() -> InvestmentObjectives {
        Self::Trading
    }
}
