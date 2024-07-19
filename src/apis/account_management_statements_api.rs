/*
 * IB REST API
 *
 * The IB REST API reference documentation
 *
 * The version of the OpenAPI document: 2.9.0
 * Contact: api@interactivebrokers.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};


/// struct for typed errors of method [`gw_api_v1_statements_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GwApiV1StatementsPostError {
    Status400(models::MissingRequiredParameterResponse),
    Status401(models::InvalidAccessTokenResponse),
    Status402(models::UnauthorizedAccessResponse),
    Status403(models::InsufficientScopeResponse),
    Status500(models::InternalServerErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Allowed scopes: [statements.read]<br>Security policy: MEDIUM
pub async fn gw_api_v1_statements_post(configuration: &configuration::Configuration, authorization: &str, stmt_request: models::StmtRequest) -> Result<models::GetStatementsResponse, Error<GwApiV1StatementsPostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/gw/api/v1/statements", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("authorization", authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&stmt_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GwApiV1StatementsPostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

