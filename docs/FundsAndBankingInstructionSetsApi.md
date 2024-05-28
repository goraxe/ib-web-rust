# \FundsAndBankingInstructionSetsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_instruction_sets_client_id_instruction_set_id_get**](FundsAndBankingInstructionSetsApi.md#gw_api_v1_instruction_sets_client_id_instruction_set_id_get) | **GET** /gw/api/v1/instruction-sets/{clientId}/{instructionSetId} | Poll status of multiple Internal Cash Transfers for given instruction set id.



## gw_api_v1_instruction_sets_client_id_instruction_set_id_get

> models::BulkMultiStatusResponse gw_api_v1_instruction_sets_client_id_instruction_set_id_get(client_id, instruction_set_id)
Poll status of multiple Internal Cash Transfers for given instruction set id.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**instruction_set_id** | **i32** | The target instruction set id. | [required] |

### Return type

[**models::BulkMultiStatusResponse**](BulkMultiStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

