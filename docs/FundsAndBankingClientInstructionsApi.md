# \FundsAndBankingClientInstructionsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_client_instructions_client_id_client_instruction_id_get**](FundsAndBankingClientInstructionsApi.md#gw_api_v1_client_instructions_client_id_client_instruction_id_get) | **GET** /gw/api/v1/client-instructions/{clientId}/{clientInstructionId} | Poll status of an Internal Cash Transfer for given instruction id.



## gw_api_v1_client_instructions_client_id_client_instruction_id_get

> models::InstructionResponse gw_api_v1_client_instructions_client_id_client_instruction_id_get(client_id, client_instruction_id)
Poll status of an Internal Cash Transfer for given instruction id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**client_instruction_id** | **i32** | The target instruction id. | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

