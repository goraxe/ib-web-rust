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
pub struct AttachedFileType {
    #[serde(rename = "fileName", skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    #[serde(rename = "fileLength", skip_serializing_if = "Option::is_none")]
    pub file_length: Option<i32>,
    #[serde(rename = "sha1Checksum", skip_serializing_if = "Option::is_none")]
    pub sha1_checksum: Option<String>,
}

impl AttachedFileType {
    pub fn new() -> AttachedFileType {
        AttachedFileType {
            file_name: None,
            file_length: None,
            sha1_checksum: None,
        }
    }
}

