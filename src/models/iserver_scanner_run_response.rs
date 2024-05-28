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
pub struct IserverScannerRunResponse {
    /// Contains contracts related to the market scanner request.
    #[serde(rename = "contracts", skip_serializing_if = "Option::is_none")]
    pub contracts: Option<Vec<models::IserverScannerRunResponseContractsInner>>,
    /// Internal Use Only
    #[serde(rename = "scan_data_column_name", skip_serializing_if = "Option::is_none")]
    pub scan_data_column_name: Option<String>,
}

impl IserverScannerRunResponse {
    pub fn new() -> IserverScannerRunResponse {
        IserverScannerRunResponse {
            contracts: None,
            scan_data_column_name: None,
        }
    }
}

