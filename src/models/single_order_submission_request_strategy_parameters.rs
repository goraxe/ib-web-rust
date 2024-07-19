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

/// SingleOrderSubmissionRequestStrategyParameters : Parameters governing the selected algorithm, if applicable.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct SingleOrderSubmissionRequestStrategyParameters {
    /// Placeholder -- these vary by algo (and not always type string, sometimes bool)
    #[serde(rename = "PLACEHOLDER", skip_serializing_if = "Option::is_none")]
    pub placeholder: Option<String>,
}

impl SingleOrderSubmissionRequestStrategyParameters {
    /// Parameters governing the selected algorithm, if applicable.
    pub fn new() -> SingleOrderSubmissionRequestStrategyParameters {
        SingleOrderSubmissionRequestStrategyParameters {
            placeholder: None,
        }
    }
}

