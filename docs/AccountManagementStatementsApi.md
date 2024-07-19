# \AccountManagementStatementsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_statements_post**](AccountManagementStatementsApi.md#gw_api_v1_statements_post) | **POST** /gw/api/v1/statements | Generates statements in supported formats based on request parameters.



## gw_api_v1_statements_post

> models::GetStatementsResponse gw_api_v1_statements_post(authorization, stmt_request)
Generates statements in supported formats based on request parameters.

Allowed scopes: [statements.read]<br>Security policy: MEDIUM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [required] |
**stmt_request** | [**StmtRequest**](StmtRequest.md) | Report request object | [required] |

### Return type

[**models::GetStatementsResponse**](GetStatementsResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, Content-Encoding, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

