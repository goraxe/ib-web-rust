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

/// UserAccountsResponseAcctProps : Returns an json object for each accessible account’s properties.
#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserAccountsResponseAcctProps {
    #[serde(rename = "U1234567", skip_serializing_if = "Option::is_none")]
    pub u1234567: Option<Box<models::UserAccountsResponseAcctPropsU1234567>>,
}

impl UserAccountsResponseAcctProps {
    /// Returns an json object for each accessible account’s properties.
    pub fn new() -> UserAccountsResponseAcctProps {
        UserAccountsResponseAcctProps {
            u1234567: None,
        }
    }
}

