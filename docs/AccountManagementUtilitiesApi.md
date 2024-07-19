# \AccountManagementUtilitiesApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**gw_api_v1_enumerations_enumeration_type_get**](AccountManagementUtilitiesApi.md#gw_api_v1_enumerations_enumeration_type_get) | **GET** /gw/api/v1/enumerations/{enumerationType} | Get enumerations for Trading permission, Employment detail, Affiliation detail, Financial Range information, ACATS and so on
[**gw_api_v1_fee_templates_client_id_post**](AccountManagementUtilitiesApi.md#gw_api_v1_fee_templates_client_id_post) | **POST** /gw/api/v1/fee-templates/{clientId} | Initiate a fee template request for provided account
[**gw_api_v1_fee_templates_query_client_id_post**](AccountManagementUtilitiesApi.md#gw_api_v1_fee_templates_query_client_id_post) | **POST** /gw/api/v1/fee-templates/query/{clientId} | Get fee template details
[**gw_api_v1_participating_banks_client_id_get**](AccountManagementUtilitiesApi.md#gw_api_v1_participating_banks_client_id_get) | **GET** /gw/api/v1/participating-banks/{clientId} | Get list of participating banks
[**gw_api_v1_requests_request_id_status_get**](AccountManagementUtilitiesApi.md#gw_api_v1_requests_request_id_status_get) | **GET** /gw/api/v1/requests/{requestId}/status | Get status of a request



## gw_api_v1_enumerations_enumeration_type_get

> models::EnumerationResponse gw_api_v1_enumerations_enumeration_type_get(enumeration_type, currency, ib_entity, md_status_non_pro, form_number)
Get enumerations for Trading permission, Employment detail, Affiliation detail, Financial Range information, ACATS and so on

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**enumeration_type** | **String** |  | [required] |
**currency** | Option<**String**> |  |  |
**ib_entity** | Option<**String**> |  |  |
**md_status_non_pro** | Option<**String**> |  |  |
**form_number** | Option<**String**> |  |  |

### Return type

[**models::EnumerationResponse**](EnumerationResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_fee_templates_client_id_post

> models::InstructionResponse gw_api_v1_fee_templates_client_id_post(client_id, gw_api_v1_fee_templates_client_id_post_request)
Initiate a fee template request for provided account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_fee_templates_client_id_post_request** | [**GwApiV1FeeTemplatesClientIdPostRequest**](GwApiV1FeeTemplatesClientIdPostRequest.md) |  | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## gw_api_v1_fee_templates_query_client_id_post

> models::InstructionResponse gw_api_v1_fee_templates_query_client_id_post(client_id, gw_api_v1_fee_templates_query_client_id_post_request)
Get fee template details

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**client_id** | **String** | The client's clientId | [required] |
**gw_api_v1_fee_templates_query_client_id_post_request** | [**GwApiV1FeeTemplatesQueryClientIdPostRequest**](GwApiV1FeeTemplatesQueryClientIdPostRequest.md) | Create fee template request body | [required] |

### Return type

[**models::InstructionResponse**](InstructionResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/problem+json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


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


## gw_api_v1_requests_request_id_status_get

> models::GwApiV1RequestsRequestIdStatusGet200Response gw_api_v1_requests_request_id_status_get(request_id, r#type)
Get status of a request

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**request_id** | **String** |  | [required] |
**r#type** | **String** |  | [required] |

### Return type

[**models::GwApiV1RequestsRequestIdStatusGet200Response**](_gw_api_v1_requests__requestId__status_get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, */*

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

