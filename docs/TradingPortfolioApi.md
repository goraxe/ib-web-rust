# \TradingPortfolioApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**portfolio_account_id_allocation_get**](TradingPortfolioApi.md#portfolio_account_id_allocation_get) | **GET** /portfolio/{accountId}/allocation | Get an account's allocations by asset class, sector group, and sector.
[**portfolio_account_id_ledger_get**](TradingPortfolioApi.md#portfolio_account_id_ledger_get) | **GET** /portfolio/{accountId}/ledger | Get ledger data for the given account.
[**portfolio_account_id_meta_get**](TradingPortfolioApi.md#portfolio_account_id_meta_get) | **GET** /portfolio/{accountId}/meta | Get an account's attributes.
[**portfolio_account_id_position_conid_get**](TradingPortfolioApi.md#portfolio_account_id_position_conid_get) | **GET** /portfolio/{accountId}/position/{conid} | Get position for a given instrument in a single account.
[**portfolio_account_id_positions_invalidate_post**](TradingPortfolioApi.md#portfolio_account_id_positions_invalidate_post) | **POST** /portfolio/{accountId}/positions/invalidate | Instructs IB to discard cached portfolio positions for a given account.
[**portfolio_account_id_positions_page_id_get**](TradingPortfolioApi.md#portfolio_account_id_positions_page_id_get) | **GET** /portfolio/{accountId}/positions/{pageId} | Get all positions in an account.
[**portfolio_account_id_summary_get**](TradingPortfolioApi.md#portfolio_account_id_summary_get) | **GET** /portfolio/{accountId}/summary | Portfolio account summary
[**portfolio_accounts_get**](TradingPortfolioApi.md#portfolio_accounts_get) | **GET** /portfolio/accounts | accounts
[**portfolio_positions_conid_get**](TradingPortfolioApi.md#portfolio_positions_conid_get) | **GET** /portfolio/positions/{conid} | Get positions in accounts for a given instrument
[**portfolio_subaccounts_get**](TradingPortfolioApi.md#portfolio_subaccounts_get) | **GET** /portfolio/subaccounts | Get attributes of subaccounts in account structure



## portfolio_account_id_allocation_get

> models::PortfolioAllocations portfolio_account_id_allocation_get(account_id, model)
Get an account's allocations by asset class, sector group, and sector.

Get an account's allocations by asset class, sector group, and sector.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**model** | Option<**String**> |  |  |

### Return type

[**models::PortfolioAllocations**](portfolioAllocations.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_ledger_get

> std::collections::HashMap<String, models::LedgerValue> portfolio_account_id_ledger_get(account_id)
Get ledger data for the given account.

Get the given account's ledger data detailing its balances by currency.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**std::collections::HashMap<String, models::LedgerValue>**](ledger_value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_meta_get

> models::AccountAttributes portfolio_account_id_meta_get(account_id)
Get an account's attributes.

Get a single account's attributes and capabilities.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::AccountAttributes**](accountAttributes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_position_conid_get

> Vec<models::IndividualPosition> portfolio_account_id_position_conid_get(account_id, conid)
Get position for a given instrument in a single account.

Get position for a given instrument in a single account. WaitSecDef attribute is always defaulted to false. It is possible to get position without security definition.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**conid** | **String** |  | [required] |

### Return type

[**Vec<models::IndividualPosition>**](individualPosition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_positions_invalidate_post

> models::PortfolioAccountIdPositionsInvalidatePost200Response portfolio_account_id_positions_invalidate_post(account_id)
Instructs IB to discard cached portfolio positions for a given account.

Instructs IB to discard cached portfolio positions for a given account, so that the next request for positions delivers freshly obtained data.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::PortfolioAccountIdPositionsInvalidatePost200Response**](_portfolio__accountId__positions_invalidate_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_positions_page_id_get

> Vec<models::IndividualPosition> portfolio_account_id_positions_page_id_get(account_id, page_id, model, sort, direction, wait_for_sec_def)
Get all positions in an account.

Get all positions in an account.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |
**page_id** | Option<**String**> |  |  |
**model** | Option<**String**> | Name of model |  |
**sort** | Option<**String**> | sorting of result positions by specified field. Defaulted to \"name\" field. |  |
**direction** | Option<**String**> | Sorting direction. Possible values \"a\" - ascending, \"d\" - descending. Defaulted to \"a\" |  |
**wait_for_sec_def** | Option<**bool**> |  |  |

### Return type

[**Vec<models::IndividualPosition>**](individualPosition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_account_id_summary_get

> models::PortfolioSummary portfolio_account_id_summary_get(account_id)
Portfolio account summary

Portfolio account summary

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** |  | [required] |

### Return type

[**models::PortfolioSummary**](portfolioSummary.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_accounts_get

> Vec<models::AccountAttributes> portfolio_accounts_get()
accounts

return accounts

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AccountAttributes>**](accountAttributes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_positions_conid_get

> std::collections::HashMap<String, models::IndividualPosition> portfolio_positions_conid_get(conid)
Get positions in accounts for a given instrument

Get positions in accounts for a given instrument (no secDef await control)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**conid** | **String** |  | [required] |

### Return type

[**std::collections::HashMap<String, models::IndividualPosition>**](individualPosition.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## portfolio_subaccounts_get

> Vec<models::AccountAttributes> portfolio_subaccounts_get()
Get attributes of subaccounts in account structure

Retrieve attributes of the subaccounts in the account structure.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<models::AccountAttributes>**](accountAttributes.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

