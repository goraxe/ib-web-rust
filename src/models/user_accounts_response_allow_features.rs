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
pub struct UserAccountsResponseAllowFeatures {
    #[serde(rename = "showGFIS", skip_serializing_if = "Option::is_none")]
    pub show_gfis: Option<bool>,
    #[serde(rename = "showEUCostReport", skip_serializing_if = "Option::is_none")]
    pub show_eu_cost_report: Option<bool>,
    #[serde(rename = "allowEventContract", skip_serializing_if = "Option::is_none")]
    pub allow_event_contract: Option<bool>,
    #[serde(rename = "allowFXConv", skip_serializing_if = "Option::is_none")]
    pub allow_fx_conv: Option<bool>,
    #[serde(rename = "allowFinancialLens", skip_serializing_if = "Option::is_none")]
    pub allow_financial_lens: Option<bool>,
    #[serde(rename = "allowMTA", skip_serializing_if = "Option::is_none")]
    pub allow_mta: Option<bool>,
    #[serde(rename = "allowTypeAhead", skip_serializing_if = "Option::is_none")]
    pub allow_type_ahead: Option<bool>,
    #[serde(rename = "allowEventTrading", skip_serializing_if = "Option::is_none")]
    pub allow_event_trading: Option<bool>,
    #[serde(rename = "snapshotRefreshTimeout", skip_serializing_if = "Option::is_none")]
    pub snapshot_refresh_timeout: Option<i64>,
    #[serde(rename = "liteUser", skip_serializing_if = "Option::is_none")]
    pub lite_user: Option<bool>,
    #[serde(rename = "showWebNews", skip_serializing_if = "Option::is_none")]
    pub show_web_news: Option<bool>,
    #[serde(rename = "research", skip_serializing_if = "Option::is_none")]
    pub research: Option<bool>,
    #[serde(rename = "debugPnl", skip_serializing_if = "Option::is_none")]
    pub debug_pnl: Option<bool>,
    #[serde(rename = "showTaxOpt", skip_serializing_if = "Option::is_none")]
    pub show_tax_opt: Option<bool>,
    #[serde(rename = "showImpactDashboard", skip_serializing_if = "Option::is_none")]
    pub show_impact_dashboard: Option<bool>,
    #[serde(rename = "allowDynAccount", skip_serializing_if = "Option::is_none")]
    pub allow_dyn_account: Option<bool>,
    #[serde(rename = "allowCrypto", skip_serializing_if = "Option::is_none")]
    pub allow_crypto: Option<bool>,
    #[serde(rename = "allowedAssetTypes", skip_serializing_if = "Option::is_none")]
    pub allowed_asset_types: Option<String>,
}

impl UserAccountsResponseAllowFeatures {
    pub fn new() -> UserAccountsResponseAllowFeatures {
        UserAccountsResponseAllowFeatures {
            show_gfis: None,
            show_eu_cost_report: None,
            allow_event_contract: None,
            allow_fx_conv: None,
            allow_financial_lens: None,
            allow_mta: None,
            allow_type_ahead: None,
            allow_event_trading: None,
            snapshot_refresh_timeout: None,
            lite_user: None,
            show_web_news: None,
            research: None,
            debug_pnl: None,
            show_tax_opt: None,
            show_impact_dashboard: None,
            allow_dyn_account: None,
            allow_crypto: None,
            allowed_asset_types: None,
        }
    }
}

