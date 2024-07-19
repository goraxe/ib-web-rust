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
pub struct AlertCreationRequest {
    /// optional; used in case of modification and represent Alert Id
    #[serde(rename = "orderId", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<i64>,
    /// Alert name.
    #[serde(rename = "alertName")]
    pub alert_name: String,
    /// Alert message which will be sent
    #[serde(rename = "alertMessage")]
    pub alert_message: String,
    /// Boolean number (0, 1) signifies if an alert can be triggered more than once. A value of ‘1’ is required for MTA alerts
    #[serde(rename = "alertRepeatable")]
    pub alert_repeatable: i32,
    /// Email address you want to send email alerts to
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// Used with a tif of “GTD” only. Signifies time when the alert should terminate if no alert is triggered.
    #[serde(rename = "expireTime", skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    /// allow (0) or disallow (1) alerts to trigger alerts through the mobile app
    #[serde(rename = "iTWSOrdersOnly", skip_serializing_if = "Option::is_none")]
    pub i_tws_orders_only: Option<i32>,
    /// Allow (1) or disallow (0) the alert to be triggered outside of regular trading hours
    #[serde(rename = "outsideRth")]
    pub outside_rth: i32,
    /// allow (1) or disallow (0) alerts to trigger email messages
    #[serde(rename = "sendMessage", skip_serializing_if = "Option::is_none")]
    pub send_message: Option<i32>,
    /// allow (1) or disallow (0) alerts to trigger TWS Pop-up messages
    #[serde(rename = "showPopup", skip_serializing_if = "Option::is_none")]
    pub show_popup: Option<i32>,
    /// Time in Force duration of alert.
    #[serde(rename = "tif")]
    pub tif: Tif,
    /// Container for all conditions applied for an alert to trigger.
    #[serde(rename = "conditions")]
    pub conditions: Vec<String>,
}

impl AlertCreationRequest {
    pub fn new(alert_name: String, alert_message: String, alert_repeatable: i32, outside_rth: i32, tif: Tif, conditions: Vec<String>) -> AlertCreationRequest {
        AlertCreationRequest {
            order_id: None,
            alert_name,
            alert_message,
            alert_repeatable,
            email: None,
            expire_time: None,
            i_tws_orders_only: None,
            outside_rth,
            send_message: None,
            show_popup: None,
            tif,
            conditions,
        }
    }
}
/// Time in Force duration of alert.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Tif {
    #[serde(rename = "GTC")]
    Gtc,
    #[serde(rename = "GTD")]
    Gtd,
}

impl Default for Tif {
    fn default() -> Tif {
        Self::Gtc
    }
}

