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
pub struct Capability {
    #[serde(rename = "code", skip_serializing_if = "Option::is_none")]
    pub code: Option<Code>,
}

impl Capability {
    pub fn new() -> Capability {
        Capability { code: None }
    }
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Code {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "FOP")]
    Fop,
    #[serde(rename = "FUND")]
    Fund,
    #[serde(rename = "FUT")]
    Fut,
    #[serde(rename = "MRGN")]
    Mrgn,
    #[serde(rename = "MULT")]
    Mult,
    #[serde(rename = "OPT")]
    Opt,
    #[serde(rename = "SSF")]
    Ssf,
    #[serde(rename = "CFD")]
    Cfd,
    #[serde(rename = "STK")]
    Stk,
    #[serde(rename = "CLP")]
    Clp,
    #[serde(rename = "LEVFX")]
    Levfx,
    #[serde(rename = "CMDTY")]
    Cmdty,
}

impl Default for Code {
    fn default() -> Code {
        Self::Bond
    }
}
