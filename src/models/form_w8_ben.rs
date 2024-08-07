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
pub struct FormW8Ben {
    #[serde(rename = "localTaxForms", skip_serializing_if = "Option::is_none")]
    pub local_tax_forms: Option<Vec<models::LocalTaxForm>>,
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "tin", skip_serializing_if = "Option::is_none")]
    pub tin: Option<String>,
    #[serde(rename = "foreignTaxId", skip_serializing_if = "Option::is_none")]
    pub foreign_tax_id: Option<String>,
    #[serde(rename = "tinOrExplanationRequired", skip_serializing_if = "Option::is_none")]
    pub tin_or_explanation_required: Option<bool>,
    #[serde(rename = "explanation", skip_serializing_if = "Option::is_none")]
    pub explanation: Option<Explanation>,
    #[serde(rename = "referenceNumber", skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<i32>,
    #[serde(rename = "part29ACountry", skip_serializing_if = "Option::is_none")]
    pub part29_a_country: Option<String>,
    #[serde(rename = "cert", skip_serializing_if = "Option::is_none")]
    pub cert: Option<bool>,
    #[serde(rename = "signatureType", skip_serializing_if = "Option::is_none")]
    pub signature_type: Option<SignatureType>,
    #[serde(rename = "blankForm", skip_serializing_if = "Option::is_none")]
    pub blank_form: Option<bool>,
    #[serde(rename = "taxFormFile", skip_serializing_if = "Option::is_none")]
    pub tax_form_file: Option<String>,
    #[serde(rename = "proprietaryFormNumber", skip_serializing_if = "Option::is_none")]
    pub proprietary_form_number: Option<i32>,
    #[serde(rename = "electronicFormat", skip_serializing_if = "Option::is_none")]
    pub electronic_format: Option<bool>,
    #[serde(rename = "submitDate", skip_serializing_if = "Option::is_none")]
    pub submit_date: Option<String>,
}

impl FormW8Ben {
    pub fn new() -> FormW8Ben {
        FormW8Ben {
            local_tax_forms: None,
            name: None,
            tin: None,
            foreign_tax_id: None,
            tin_or_explanation_required: None,
            explanation: None,
            reference_number: None,
            part29_a_country: None,
            cert: None,
            signature_type: None,
            blank_form: None,
            tax_form_file: None,
            proprietary_form_number: None,
            electronic_format: None,
            submit_date: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Explanation {
    #[serde(rename = "US_TIN")]
    UsTin,
    #[serde(rename = "TIN_NOT_DISCLOSED")]
    TinNotDisclosed,
    #[serde(rename = "TIN_NOT_REQUIRED")]
    TinNotRequired,
    #[serde(rename = "TIN_NOT_ISSUED")]
    TinNotIssued,
}

impl Default for Explanation {
    fn default() -> Explanation {
        Self::UsTin
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum SignatureType {
    #[serde(rename = "Electronic")]
    Electronic,
    #[serde(rename = "Physical")]
    Physical,
}

impl Default for SignatureType {
    fn default() -> SignatureType {
        Self::Electronic
    }
}

