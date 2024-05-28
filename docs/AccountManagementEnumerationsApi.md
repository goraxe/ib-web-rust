# \AccountManagementEnumerationsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_enumerations_enumeration_type_get**](AccountManagementEnumerationsApi.md#gw_api_v1_enumerations_enumeration_type_get) | **GET** /gw/api/v1/enumerations/{enumerationType} | 



## gw_api_v1_enumerations_enumeration_type_get

> serde_json::Value gw_api_v1_enumerations_enumeration_type_get(enumeration_type, currency, ib_entity, md_status_non_pro, form_number)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enumeration_type** | **String** |  | [required] |
**currency** | Option<**String**> |  |  |
**ib_entity** | Option<**String**> |  |  |
**md_status_non_pro** | Option<**String**> |  |  |
**form_number** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

