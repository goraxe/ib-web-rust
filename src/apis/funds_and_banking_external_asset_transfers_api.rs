/*
 * IB REST API
 *
 * The IB REST API reference documentation
 *
 * The version of the OpenAPI document: 2.7.0
 * Contact: api@interactivebrokers.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`gw_api_v1_external_asset_transfers_client_id_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GwApiV1ExternalAssetTransfersClientIdPostError {
    Status400(models::MissingRequiredParameterResponse),
    Status403(models::BusinessRejectResponse),
    Status422(models::BusinessRejectResponse),
    Status500(models::InternalServerErrorResponse),
    UnknownValue(serde_json::Value),
}


pub async fn gw_api_v1_external_asset_transfers_client_id_post(configuration: &configuration::Configuration, client_id: &str, gw_api_v1_external_asset_transfers_client_id_post_request: models::GwApiV1ExternalAssetTransfersClientIdPostRequest) -> Result<models::InstructionResponse, Error<GwApiV1ExternalAssetTransfersClientIdPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/gw/api/v1/external-asset-transfers/{clientId}", local_var_configuration.base_path, clientId=crate::apis::urlencode(client_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&gw_api_v1_external_asset_transfers_client_id_post_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GwApiV1ExternalAssetTransfersClientIdPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

