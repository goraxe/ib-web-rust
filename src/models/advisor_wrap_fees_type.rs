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
pub struct AdvisorWrapFeesType {
    #[serde(rename = "automatedFeesDetails", skip_serializing_if = "Option::is_none")]
    pub automated_fees_details: Option<Vec<models::AutomatedWrapFeeDetailsType>>,
    #[serde(rename = "highWaterMarkConfigHwma", skip_serializing_if = "Option::is_none")]
    pub high_water_mark_config_hwma: Option<Box<models::HighWaterMarkType>>,
    #[serde(rename = "highWaterMarkConfigHwmq", skip_serializing_if = "Option::is_none")]
    pub high_water_mark_config_hwmq: Option<Box<models::HighWaterMarkType>>,
    #[serde(rename = "strategy", skip_serializing_if = "Option::is_none")]
    pub strategy: Option<Strategy>,
    #[serde(rename = "chargeAdvisor", skip_serializing_if = "Option::is_none")]
    pub charge_advisor: Option<bool>,
    #[serde(rename = "chargeOtherFeesToAdvisor", skip_serializing_if = "Option::is_none")]
    pub charge_other_fees_to_advisor: Option<bool>,
}

impl AdvisorWrapFeesType {
    pub fn new() -> AdvisorWrapFeesType {
        AdvisorWrapFeesType {
            automated_fees_details: None,
            high_water_mark_config_hwma: None,
            high_water_mark_config_hwmq: None,
            strategy: None,
            charge_advisor: None,
            charge_other_fees_to_advisor: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Strategy {
    #[serde(rename = "AUTOMATED")]
    Automated,
    #[serde(rename = "DIRECTBILLING")]
    Directbilling,
    #[serde(rename = "NO_FEE")]
    NoFee,
}

impl Default for Strategy {
    fn default() -> Strategy {
        Self::Automated
    }
}

