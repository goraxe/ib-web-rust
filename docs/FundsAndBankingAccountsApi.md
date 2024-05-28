# \FundsAndBankingAccountsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_accounts_close_client_id_post**](FundsAndBankingAccountsApi.md#gw_api_v1_accounts_close_client_id_post) | **POST** /gw/api/v1/accounts/close/{clientId} | Initiate account closure request for the provided account



## gw_api_v1_accounts_close_client_id_post

> models::InstructionResponse gw_api_v1_accounts_close_client_id_post(client_id, gw_api_v1_accounts_close_client_id_post_request)
Initiate account closure request for the provided account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_accounts_close_client_id_post_request** | [**GwApiV1AccountsCloseClientIdPostRequest**](GwApiV1AccountsCloseClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

