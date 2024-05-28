# \AccountManagementAccountsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_accounts_account_id_details_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_details_get) | **GET** /gw/api/v1/accounts/{accountId}/details | 
[**gw_api_v1_accounts_account_id_kyc_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_kyc_get) | **GET** /gw/api/v1/accounts/{accountId}/kyc | 
[**gw_api_v1_accounts_account_id_login_messages_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_login_messages_get) | **GET** /gw/api/v1/accounts/{accountId}/login-messages | 
[**gw_api_v1_accounts_account_id_status_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_status_get) | **GET** /gw/api/v1/accounts/{accountId}/status | 
[**gw_api_v1_accounts_account_id_tasks_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_tasks_get) | **GET** /gw/api/v1/accounts/{accountId}/tasks | 
[**gw_api_v1_accounts_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_get) | **GET** /gw/api/v1/accounts | 
[**gw_api_v1_accounts_patch**](AccountManagementAccountsApi.md#gw_api_v1_accounts_patch) | **PATCH** /gw/api/v1/accounts | 
[**gw_api_v1_accounts_post**](AccountManagementAccountsApi.md#gw_api_v1_accounts_post) | **POST** /gw/api/v1/accounts | 



## gw_api_v1_accounts_account_id_details_get

> models::AccountDetailsResponse gw_api_v1_accounts_account_id_details_get(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::AccountDetailsResponse**](AccountDetailsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_kyc_get

> Vec<models::Au10TixDetailResponse> gw_api_v1_accounts_account_id_kyc_get(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**Vec<models::Au10TixDetailResponse>**](Au10TixDetailResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_login_messages_get

> models::LoginMessageResponse gw_api_v1_accounts_account_id_login_messages_get(account_id, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**r#type** | Option<**String**> |  |  |

### Return type

[**models::LoginMessageResponse**](LoginMessageResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_status_get

> models::AccountStatusResponse gw_api_v1_accounts_account_id_status_get(account_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::AccountStatusResponse**](AccountStatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_tasks_get

> serde_json::Value gw_api_v1_accounts_account_id_tasks_get(account_id, r#type)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**r#type** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_get

> serde_json::Value gw_api_v1_accounts_get(account_id, external_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> |  |  |
**external_id** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_patch

> models::UpsertResponse gw_api_v1_accounts_patch(account_management_requests_payload, authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_management_requests_payload** | [**AccountManagementRequestsPayload**](AccountManagementRequestsPayload.md) |  | [required] |
**authorization** | Option<**String**> |  |  |

### Return type

[**models::UpsertResponse**](UpsertResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/jwt
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_post

> models::UpsertResponse gw_api_v1_accounts_post(application_payload, authorization)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_payload** | [**ApplicationPayload**](ApplicationPayload.md) |  | [required] |
**authorization** | Option<**String**> |  |  |

### Return type

[**models::UpsertResponse**](UpsertResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/jwt
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

