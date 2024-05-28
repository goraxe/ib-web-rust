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


/// struct for typed errors of method [`portfolio_account_id_allocation_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountIdAllocationGetError {
    Status400(models::ErrorResponse),
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_account_id_ledger_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountIdLedgerGetError {
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_account_id_meta_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountIdMetaGetError {
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_account_id_position_conid_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountIdPositionConidGetError {
    Status400(models::ErrorResponse),
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_account_id_positions_invalidate_post`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountIdPositionsInvalidatePostError {
    Status400(models::ErrorResponse),
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_account_id_positions_page_id_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountIdPositionsPageIdGetError {
    Status400(models::ErrorResponse),
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_account_id_summary_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountIdSummaryGetError {
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_accounts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioAccountsGetError {
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_positions_conid_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioPositionsConidGetError {
    Status400(models::ErrorResponse),
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`portfolio_subaccounts_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PortfolioSubaccountsGetError {
    Status401(String),
    Status500(models::ErrorOnlyResponse),
    Status503(models::ErrorResponse),
    UnknownValue(serde_json::Value),
}


/// Get an account's allocations by asset class, sector group, and sector.
pub async fn portfolio_account_id_allocation_get(configuration: &configuration::Configuration, account_id: &str, model: Option<models::serde_json::Value>) -> Result<models::PortfolioAllocations, Error<PortfolioAccountIdAllocationGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/{accountId}/allocation", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = model {
        local_var_req_builder = local_var_req_builder.query(&[("model", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountIdAllocationGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the given account's ledger data detailing its balances by currency.
pub async fn portfolio_account_id_ledger_get(configuration: &configuration::Configuration, account_id: &str) -> Result<std::collections::HashMap<String, models::LedgerValue>, Error<PortfolioAccountIdLedgerGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/{accountId}/ledger", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountIdLedgerGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get a single account's attributes and capabilities.
pub async fn portfolio_account_id_meta_get(configuration: &configuration::Configuration, account_id: &str) -> Result<models::AccountAttributes, Error<PortfolioAccountIdMetaGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/{accountId}/meta", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountIdMetaGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get position for a given instrument in a single account. WaitSecDef attribute is always defaulted to false. It is possible to get position without security definition.
pub async fn portfolio_account_id_position_conid_get(configuration: &configuration::Configuration, account_id: &str, conid: &str) -> Result<Vec<models::IndividualPosition>, Error<PortfolioAccountIdPositionConidGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/{accountId}/position/{conid}", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id), conid=crate::apis::urlencode(conid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountIdPositionConidGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Instructs IB to discard cached portfolio positions for a given account, so that the next request for positions delivers freshly obtained data.
pub async fn portfolio_account_id_positions_invalidate_post(configuration: &configuration::Configuration, account_id: &str) -> Result<models::PortfolioAccountIdPositionsInvalidatePost200Response, Error<PortfolioAccountIdPositionsInvalidatePostError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/{accountId}/positions/invalidate", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountIdPositionsInvalidatePostError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get all positions in an account.
pub async fn portfolio_account_id_positions_page_id_get(configuration: &configuration::Configuration, account_id: &str, page_id: Option<&str>, model: Option<&str>, sort: Option<&str>, direction: Option<&str>, wait_for_sec_def: Option<bool>) -> Result<Vec<models::IndividualPosition>, Error<PortfolioAccountIdPositionsPageIdGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/{accountId}/positions/{pageId}", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id), pageId=crate::apis::urlencode(page_id.unwrap()));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = model {
        local_var_req_builder = local_var_req_builder.query(&[("model", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = sort {
        local_var_req_builder = local_var_req_builder.query(&[("sort", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = direction {
        local_var_req_builder = local_var_req_builder.query(&[("direction", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wait_for_sec_def {
        local_var_req_builder = local_var_req_builder.query(&[("waitForSecDef", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountIdPositionsPageIdGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Portfolio account summary
pub async fn portfolio_account_id_summary_get(configuration: &configuration::Configuration, account_id: &str) -> Result<models::PortfolioSummary, Error<PortfolioAccountIdSummaryGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/{accountId}/summary", local_var_configuration.base_path, accountId=crate::apis::urlencode(account_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountIdSummaryGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// return accounts
pub async fn portfolio_accounts_get(configuration: &configuration::Configuration, ) -> Result<Vec<models::AccountAttributes>, Error<PortfolioAccountsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/accounts", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioAccountsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get positions in accounts for a given instrument (no secDef await control)
pub async fn portfolio_positions_conid_get(configuration: &configuration::Configuration, conid: &str) -> Result<std::collections::HashMap<String, models::IndividualPosition>, Error<PortfolioPositionsConidGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/positions/{conid}", local_var_configuration.base_path, conid=crate::apis::urlencode(conid));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioPositionsConidGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Retrieve attributes of the subaccounts in the account structure.
pub async fn portfolio_subaccounts_get(configuration: &configuration::Configuration, ) -> Result<Vec<models::AccountAttributes>, Error<PortfolioSubaccountsGetError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/portfolio/subaccounts", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<PortfolioSubaccountsGetError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}
