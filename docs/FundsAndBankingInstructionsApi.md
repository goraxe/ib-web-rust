# \FundsAndBankingInstructionsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_instructions_cancel_client_id_post**](FundsAndBankingInstructionsApi.md#gw_api_v1_instructions_cancel_client_id_post) | **POST** /gw/api/v1/instructions/cancel/{clientId} | Initiate a cancel instruction request for provided instructionId
[**gw_api_v1_instructions_client_id_instruction_id_get**](FundsAndBankingInstructionsApi.md#gw_api_v1_instructions_client_id_instruction_id_get) | **GET** /gw/api/v1/instructions/{clientId}/{instructionId} | Poll status of an Internal Cash Transfer for given instruction id.
[**gw_api_v1_instructions_query_client_id_post**](FundsAndBankingInstructionsApi.md#gw_api_v1_instructions_query_client_id_post) | **POST** /gw/api/v1/instructions/query/{clientId} | Get Recent Instructions



## gw_api_v1_instructions_cancel_client_id_post

> models::InstructionResponse gw_api_v1_instructions_cancel_client_id_post(client_id, gw_api_v1_instructions_cancel_client_id_post_request)
Initiate a cancel instruction request for provided instructionId

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_instructions_cancel_client_id_post_request** | [**GwApiV1InstructionsCancelClientIdPostRequest**](GwApiV1InstructionsCancelClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_instructions_client_id_instruction_id_get

> models::InstructionResponse gw_api_v1_instructions_client_id_instruction_id_get(client_id, instruction_id)
Poll status of an Internal Cash Transfer for given instruction id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**instruction_id** | **i32** | The target instruction id. | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_instructions_query_client_id_post

> models::InstructionResponse gw_api_v1_instructions_query_client_id_post(client_id, gw_api_v1_instructions_query_client_id_post_request)
Get Recent Instructions

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_instructions_query_client_id_post_request** | [**GwApiV1InstructionsQueryClientIdPostRequest**](GwApiV1InstructionsQueryClientIdPostRequest.md) | Create recent instructions request body | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

