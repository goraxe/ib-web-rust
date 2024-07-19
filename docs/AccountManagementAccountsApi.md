# \AccountManagementAccountsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_accounts_account_id_details_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_details_get) | **GET** /gw/api/v1/accounts/{accountId}/details | Get account details data i.e. capabilities, investment experience, PII data
[**gw_api_v1_accounts_account_id_kyc_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_kyc_get) | **GET** /gw/api/v1/accounts/{accountId}/kyc | Get Au10Tix Details
[**gw_api_v1_accounts_account_id_login_messages_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_login_messages_get) | **GET** /gw/api/v1/accounts/{accountId}/login-messages | Get login messages per account
[**gw_api_v1_accounts_account_id_status_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_status_get) | **GET** /gw/api/v1/accounts/{accountId}/status | Get account status
[**gw_api_v1_accounts_account_id_tasks_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_account_id_tasks_get) | **GET** /gw/api/v1/accounts/{accountId}/tasks | Get pending tasks
[**gw_api_v1_accounts_close_client_id_post**](AccountManagementAccountsApi.md#gw_api_v1_accounts_close_client_id_post) | **POST** /gw/api/v1/accounts/close/{clientId} | Initiate account closure request for the provided account
[**gw_api_v1_accounts_get**](AccountManagementAccountsApi.md#gw_api_v1_accounts_get) | **GET** /gw/api/v1/accounts | Get file details
[**gw_api_v1_accounts_patch**](AccountManagementAccountsApi.md#gw_api_v1_accounts_patch) | **PATCH** /gw/api/v1/accounts | Update existing accounts
[**gw_api_v1_accounts_post**](AccountManagementAccountsApi.md#gw_api_v1_accounts_post) | **POST** /gw/api/v1/accounts | Create new accounts



## gw_api_v1_accounts_account_id_details_get

> models::AccountDetailsResponse gw_api_v1_accounts_account_id_details_get(account_id)
Get account details data i.e. capabilities, investment experience, PII data

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
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_kyc_get

> models::Au10TixDetailResponse gw_api_v1_accounts_account_id_kyc_get(account_id)
Get Au10Tix Details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::Au10TixDetailResponse**](Au10TixDetailResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_login_messages_get

> models::LoginMessageResponse gw_api_v1_accounts_account_id_login_messages_get(account_id, r#type)
Get login messages per account

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
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_status_get

> models::AccountStatusResponse gw_api_v1_accounts_account_id_status_get(account_id)
Get account status

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
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_account_id_tasks_get

> models::GwApiV1AccountsAccountIdTasksGet200Response gw_api_v1_accounts_account_id_tasks_get(account_id, r#type)
Get pending tasks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**r#type** | Option<**String**> |  |  |

### Return type

[**models::GwApiV1AccountsAccountIdTasksGet200Response**](_gw_api_v1_accounts__accountId__tasks_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_close_client_id_post

> models::InstructionResponse gw_api_v1_accounts_close_client_id_post(client_id, gw_api_v1_accounts_close_client_id_post_request)
Initiate account closure request for the provided account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_accounts_close_client_id_post_request** | [**GwApiV1AccountsCloseClientIdPostRequest**](GwApiV1AccountsCloseClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_get

> models::GwApiV1AccountsGet200Response gw_api_v1_accounts_get(account_id, external_id)
Get file details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | Option<**String**> |  |  |
**external_id** | Option<**String**> |  |  |

### Return type

[**models::GwApiV1AccountsGet200Response**](_gw_api_v1_accounts_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_patch

> models::StatusResponse gw_api_v1_accounts_patch(account_management_requests_payload)
Update existing accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_management_requests_payload** | [**AccountManagementRequestsPayload**](AccountManagementRequestsPayload.md) |  | [required] |

### Return type

[**models::StatusResponse**](StatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/jwt
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_accounts_post

> models::StatusResponse gw_api_v1_accounts_post(application_payload)
Create new accounts

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**application_payload** | [**ApplicationPayload**](ApplicationPayload.md) |  | [required] |

### Return type

[**models::StatusResponse**](StatusResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/jwt
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

