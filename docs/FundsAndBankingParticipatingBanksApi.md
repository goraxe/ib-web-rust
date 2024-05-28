# \FundsAndBankingParticipatingBanksApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_participating_banks_client_id_get**](FundsAndBankingParticipatingBanksApi.md#gw_api_v1_participating_banks_client_id_get) | **GET** /gw/api/v1/participating-banks/{clientId} | Get list of participating banks



## gw_api_v1_participating_banks_client_id_get

> models::GetParticipatingListResponse gw_api_v1_participating_banks_client_id_get(client_id, r#type)
Get list of participating banks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**r#type** | **String** | Parameter for which the list of participating banks is fetched | [required] |

### Return type

[**models::GetParticipatingListResponse**](GetParticipatingListResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

