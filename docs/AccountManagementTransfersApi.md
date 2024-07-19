# \AccountManagementTransfersApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_external_asset_transfers_client_id_post**](AccountManagementTransfersApi.md#gw_api_v1_external_asset_transfers_client_id_post) | **POST** /gw/api/v1/external-asset-transfers/{clientId} | External Asset transfer (FOP, DWAC , Complex Asset Transfer)
[**gw_api_v1_external_cash_transfers_client_id_post**](AccountManagementTransfersApi.md#gw_api_v1_external_cash_transfers_client_id_post) | **POST** /gw/api/v1/external-cash-transfers/{clientId} | External Cash transfers (Deposit and Withdraw fund)
[**gw_api_v1_external_cash_transfers_query_client_id_post**](AccountManagementTransfersApi.md#gw_api_v1_external_cash_transfers_query_client_id_post) | **POST** /gw/api/v1/external-cash-transfers/query/{clientId} | Get Withdrawable Cash Queries
[**gw_api_v1_internal_asset_transfers_client_id_post**](AccountManagementTransfersApi.md#gw_api_v1_internal_asset_transfers_client_id_post) | **POST** /gw/api/v1/internal-asset-transfers/{clientId} | Asset transfers within IB
[**gw_api_v1_internal_cash_transfers_client_id_post**](AccountManagementTransfersApi.md#gw_api_v1_internal_cash_transfers_client_id_post) | **POST** /gw/api/v1/internal-cash-transfers/{clientId} | Cash transfer within IB



## gw_api_v1_external_asset_transfers_client_id_post

> models::InstructionResponse gw_api_v1_external_asset_transfers_client_id_post(client_id, gw_api_v1_external_asset_transfers_client_id_post_request)
External Asset transfer (FOP, DWAC , Complex Asset Transfer)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_external_asset_transfers_client_id_post_request** | [**GwApiV1ExternalAssetTransfersClientIdPostRequest**](GwApiV1ExternalAssetTransfersClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_external_cash_transfers_client_id_post

> models::InstructionResponse gw_api_v1_external_cash_transfers_client_id_post(client_id, gw_api_v1_external_cash_transfers_client_id_post_request)
External Cash transfers (Deposit and Withdraw fund)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_external_cash_transfers_client_id_post_request** | [**GwApiV1ExternalCashTransfersClientIdPostRequest**](GwApiV1ExternalCashTransfersClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_external_cash_transfers_query_client_id_post

> models::InstructionResponse gw_api_v1_external_cash_transfers_query_client_id_post(client_id, gw_api_v1_external_cash_transfers_query_client_id_post_request)
Get Withdrawable Cash Queries

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_external_cash_transfers_query_client_id_post_request** | [**GwApiV1ExternalCashTransfersQueryClientIdPostRequest**](GwApiV1ExternalCashTransfersQueryClientIdPostRequest.md) | Create fee template request body | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_internal_asset_transfers_client_id_post

> models::InstructionResponse gw_api_v1_internal_asset_transfers_client_id_post(client_id, gw_api_v1_internal_asset_transfers_client_id_post_request)
Asset transfers within IB

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_internal_asset_transfers_client_id_post_request** | [**GwApiV1InternalAssetTransfersClientIdPostRequest**](GwApiV1InternalAssetTransfersClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_internal_cash_transfers_client_id_post

> models::InstructionResponse gw_api_v1_internal_cash_transfers_client_id_post(client_id, gw_api_v1_internal_cash_transfers_client_id_post_request)
Cash transfer within IB

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_internal_cash_transfers_client_id_post_request** | [**GwApiV1InternalCashTransfersClientIdPostRequest**](GwApiV1InternalCashTransfersClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

