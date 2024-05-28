# \FundsAndBankingInternalAssetTransfersApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_internal_asset_transfers_client_id_post**](FundsAndBankingInternalAssetTransfersApi.md#gw_api_v1_internal_asset_transfers_client_id_post) | **POST** /gw/api/v1/internal-asset-transfers/{clientId} | Asset transfers within IB



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

