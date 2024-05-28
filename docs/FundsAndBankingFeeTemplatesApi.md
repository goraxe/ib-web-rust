# \FundsAndBankingFeeTemplatesApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_fee_templates_client_id_post**](FundsAndBankingFeeTemplatesApi.md#gw_api_v1_fee_templates_client_id_post) | **POST** /gw/api/v1/fee-templates/{clientId} | Initiate a fee template request for provided account
[**gw_api_v1_fee_templates_query_client_id_post**](FundsAndBankingFeeTemplatesApi.md#gw_api_v1_fee_templates_query_client_id_post) | **POST** /gw/api/v1/fee-templates/query/{clientId} | Get fee template details



## gw_api_v1_fee_templates_client_id_post

> models::InstructionResponse gw_api_v1_fee_templates_client_id_post(client_id, gw_api_v1_fee_templates_client_id_post_request)
Initiate a fee template request for provided account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_fee_templates_client_id_post_request** | [**GwApiV1FeeTemplatesClientIdPostRequest**](GwApiV1FeeTemplatesClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_fee_templates_query_client_id_post

> models::InstructionResponse gw_api_v1_fee_templates_query_client_id_post(client_id, gw_api_v1_fee_templates_query_client_id_post_request)
Get fee template details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_fee_templates_query_client_id_post_request** | [**GwApiV1FeeTemplatesQueryClientIdPostRequest**](GwApiV1FeeTemplatesQueryClientIdPostRequest.md) | Create fee template request body | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

