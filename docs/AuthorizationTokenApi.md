# \AuthorizationTokenApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**generate_token**](AuthorizationTokenApi.md#generate_token) | **POST** /oauth2/api/v1/token | Request an OAuth 2.0 access token



## generate_token

> models::TokenResponse generate_token(grant_type, client_assertion, client_assertion_type, scope)
Request an OAuth 2.0 access token

This endpoint returns OAuth 2.0 access tokens based on the request parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**grant_type** | **String** | The [authorization grant flow](https://dataetracker.ietf.org/doc/html/rfc6749#section-1.3) for the creation of the tokens. | [required] |
**client_assertion** | **String** | The compact [client assertion](https://www.rfc-editor.org/rfc/rfc7521.html) token used to authenticate you as a registered client. | [required] |
**client_assertion_type** | **String** | The [client assertion type](https://www.rfc-editor.org/rfc/rfc7521.html#section-4.2) that identifies the client assertion. | [required] |
**scope** | Option<**String**> | The space-delimited list of scopes |  |

### Return type

[**models::TokenResponse**](TokenResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/x-www-form-urlencoded
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

