# \FundsAndBankingBankInstructionsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_bank_instructions_client_id_post**](FundsAndBankingBankInstructionsApi.md#gw_api_v1_bank_instructions_client_id_post) | **POST** /gw/api/v1/bank-instructions/{clientId} | Initiate a banking instruction (currently only ACH, eDDA, Delete) for the provided IB account
[**gw_api_v1_bank_instructions_query_client_id_post**](FundsAndBankingBankInstructionsApi.md#gw_api_v1_bank_instructions_query_client_id_post) | **POST** /gw/api/v1/bank-instructions/query/{clientId} | Bank instruction Queries



## gw_api_v1_bank_instructions_client_id_post

> models::InstructionResponse gw_api_v1_bank_instructions_client_id_post(client_id, gw_api_v1_bank_instructions_client_id_post_request)
Initiate a banking instruction (currently only ACH, eDDA, Delete) for the provided IB account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_bank_instructions_client_id_post_request** | [**GwApiV1BankInstructionsClientIdPostRequest**](GwApiV1BankInstructionsClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_bank_instructions_query_client_id_post

> models::InstructionResponse gw_api_v1_bank_instructions_query_client_id_post(client_id, gw_api_v1_bank_instructions_query_client_id_post_request)
Bank instruction Queries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_bank_instructions_query_client_id_post_request** | [**GwApiV1BankInstructionsQueryClientIdPostRequest**](GwApiV1BankInstructionsQueryClientIdPostRequest.md) | Create get instruction name request body | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

