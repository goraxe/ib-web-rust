# \AuthorizationSsoSessionsApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_sso_sessions_post**](AuthorizationSsoSessionsApi.md#gw_api_v1_sso_sessions_post) | **POST** /gw/api/v1/sso-sessions | Create a new SSO session on behalf of an end-user.



## gw_api_v1_sso_sessions_post

> models::CreateSessionResponse gw_api_v1_sso_sessions_post(authorization, create_session_request)
Create a new SSO session on behalf of an end-user.

Allowed scopes: [sessions.create]<br>Security policy: MEDIUM

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization** | **String** | Specifies the authorization header value (e.g., Bearer eyJ0eXAiOiJKV1...). | [required] |
**create_session_request** | [**CreateSessionRequest**](CreateSessionRequest.md) | Create session on behalf of end-user. | [required] |

### Return type

[**models::CreateSessionResponse**](CreateSessionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/jwt
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

