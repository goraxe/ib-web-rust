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
pub struct SummaryOfAccountMarginResponseTotal {
    /// The minimum amount required to open a new position.
    #[serde(rename = "current_initial", skip_serializing_if = "Option::is_none")]
    pub current_initial: Option<String>,
    /// Provides a projected “at expiration” margin value based on the soon-to-expire contracts in your portfolio.
    #[serde(rename = "Prdctd Pst-xpry Mrgn @ Opn", skip_serializing_if = "Option::is_none")]
    pub prdctd_pst_xpry_mrgn_at__opn: Option<String>,
    /// The amount of equity required to maintain your positions.
    #[serde(rename = "current_maint", skip_serializing_if = "Option::is_none")]
    pub current_maint: Option<String>,
    /// Provides a projected \"liquid\" initial margin value based on account liquidation value.
    #[serde(rename = "projected_liquidity_inital_margin", skip_serializing_if = "Option::is_none")]
    pub projected_liquidity_inital_margin: Option<String>,
    /// If it is 3:00 pm ET, the next calculation you’re looking ahead to is after the close, or the Overnight Initial Margin. If it’s 3:00 am ET, the next calculation will be at the market’s open.  * `Securities` – Projected maintenance margin requirement as of next period’s margin change, in the base currency of the account.   * `Commodities` – Maintenance margin requirement as of next period’s margin change in the base currency of the account based on current margin requirements, which are subject to change. This value depends on when you are viewing your margin requirements. 
    #[serde(rename = "Prjctd Lk Ahd Mntnnc Mrgn", skip_serializing_if = "Option::is_none")]
    pub prjctd_lk_ahd_mntnnc_mrgn: Option<String>,
    /// Overnight refers to the window of time after the local market trading day is closed.    * Securities – Projected overnight initial margin requirement in the base currency of the account.    * Commodities – Overnight initial margin requirement in the base currency of the account based on current margin requirements, which are subject to change. 
    #[serde(rename = "projected_overnight_initial_margin", skip_serializing_if = "Option::is_none")]
    pub projected_overnight_initial_margin: Option<String>,
    /// Overnight refers to the window of time after the local market trading day is closed.    * `Securities` – Projected overnight maintenance margin requirement in the base currency of the account.    * `Commodities` – Overnight maintenance margin requirement in the base currency of the account based on current margin requirements, which are subject to change. 
    #[serde(rename = "Prjctd Ovrnght Mntnnc Mrgn", skip_serializing_if = "Option::is_none")]
    pub prjctd_ovrnght_mntnnc_mrgn: Option<String>,
}

impl SummaryOfAccountMarginResponseTotal {
    pub fn new() -> SummaryOfAccountMarginResponseTotal {
        SummaryOfAccountMarginResponseTotal {
            current_initial: None,
            prdctd_pst_xpry_mrgn_at__opn: None,
            current_maint: None,
            projected_liquidity_inital_margin: None,
            prjctd_lk_ahd_mntnnc_mrgn: None,
            projected_overnight_initial_margin: None,
            prjctd_ovrnght_mntnnc_mrgn: None,
        }
    }
}
