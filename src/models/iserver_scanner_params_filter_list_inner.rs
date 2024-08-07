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
pub struct IserverScannerParamsFilterListInner {
    /// Returns the group of filters the request is affiliated with.
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    /// Returns the human-readable identifier for the filter.
    #[serde(rename = "display_name", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Value used for the market scanner request.
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// Returns the type of value to be used in the request. This can indicate a range based value, or if it should be a single value.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// combo values when type equals to combo
    #[serde(rename = "combo_values", skip_serializing_if = "Option::is_none")]
    pub combo_values: Option<Vec<models::IserverScannerParamsFilterListInnerComboValuesInner>>,
}

impl IserverScannerParamsFilterListInner {
    pub fn new() -> IserverScannerParamsFilterListInner {
        IserverScannerParamsFilterListInner {
            group: None,
            display_name: None,
            code: None,
            r#type: None,
            combo_values: None,
        }
    }
}

