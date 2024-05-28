# \TradingOrdersApi

All URIs are relative to *https://api.ibkr.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**iserver_account_account_id_order_order_id_delete**](TradingOrdersApi.md#iserver_account_account_id_order_order_id_delete) | **DELETE** /iserver/account/{accountId}/order/{orderId} | Cancel an existing, unfilled order.
[**iserver_account_account_id_order_order_id_post**](TradingOrdersApi.md#iserver_account_account_id_order_order_id_post) | **POST** /iserver/account/{accountId}/order/{orderId} | Modify an existing, unfilled order.
[**iserver_account_account_id_orders_post**](TradingOrdersApi.md#iserver_account_account_id_orders_post) | **POST** /iserver/account/{accountId}/orders | Submit a new order(s) ticket, bracket, or OCA group.
[**iserver_account_account_id_orders_whatif_post**](TradingOrdersApi.md#iserver_account_account_id_orders_whatif_post) | **POST** /iserver/account/{accountId}/orders/whatif | Preview the projected effects of an order ticket or bracket of orders, including cost and changes to margin and account equity.
[**iserver_account_order_status_order_id_get**](TradingOrdersApi.md#iserver_account_order_status_order_id_get) | **GET** /iserver/account/order/status/{orderId} | Retrieve the status of a single order.
[**iserver_account_orders_get**](TradingOrdersApi.md#iserver_account_orders_get) | **GET** /iserver/account/orders | Retrieves open orders and filled or cancelled orders submitted during the current brokerage session.
[**iserver_account_trades_get**](TradingOrdersApi.md#iserver_account_trades_get) | **GET** /iserver/account/trades | Retrieve a list of trades.
[**iserver_notification_post**](TradingOrdersApi.md#iserver_notification_post) | **POST** /iserver/notification | Respond to a server prompt.
[**iserver_questions_suppress_post**](TradingOrdersApi.md#iserver_questions_suppress_post) | **POST** /iserver/questions/suppress | Suppress the specified order reply messages.
[**iserver_questions_suppress_reset_post**](TradingOrdersApi.md#iserver_questions_suppress_reset_post) | **POST** /iserver/questions/suppress/reset | Removes suppression of all order reply messages.
[**iserver_reply_reply_id_post**](TradingOrdersApi.md#iserver_reply_reply_id_post) | **POST** /iserver/reply/{replyId} | Confirm an order reply message.



## iserver_account_account_id_order_order_id_delete

> models::IserverAccountAccountIdOrderOrderIdDelete200Response iserver_account_account_id_order_order_id_delete(account_id, order_id)
Cancel an existing, unfilled order.

Cancel an existing, unfilled order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The account to which the order will clear. | [required] |
**order_id** | **String** | The IB-assigned order ID of the desired order ticket. | [required] |

### Return type

[**models::IserverAccountAccountIdOrderOrderIdDelete200Response**](_iserver_account__accountId__order__orderId__delete_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_order_order_id_post

> models::IserverAccountAccountIdOrderOrderIdPost200Response iserver_account_account_id_order_order_id_post(account_id, order_id, single_order_submission_request)
Modify an existing, unfilled order.

Modify an existing, unfilled order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The account to which the order will clear. | [required] |
**order_id** | **String** | The IB-assigned order ID of the desired order ticket. | [required] |
**single_order_submission_request** | [**SingleOrderSubmissionRequest**](SingleOrderSubmissionRequest.md) |  | [required] |

### Return type

[**models::IserverAccountAccountIdOrderOrderIdPost200Response**](_iserver_account__accountId__order__orderId__post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_orders_post

> models::IserverAccountAccountIdOrderOrderIdPost200Response iserver_account_account_id_orders_post(account_id, single_order_submission_request)
Submit a new order(s) ticket, bracket, or OCA group.

Submit a new order(s) ticket, bracket, or OCA group.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The account to which the order will clear. | [required] |
**single_order_submission_request** | [**Vec<models::SingleOrderSubmissionRequest>**](singleOrderSubmissionRequest.md) |  | [required] |

### Return type

[**models::IserverAccountAccountIdOrderOrderIdPost200Response**](_iserver_account__accountId__order__orderId__post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_account_id_orders_whatif_post

> models::OrderPreview iserver_account_account_id_orders_whatif_post(account_id, single_order_submission_request)
Preview the projected effects of an order ticket or bracket of orders, including cost and changes to margin and account equity.

Preview the projected effects of an order ticket or bracket of orders, including cost and changes to margin and account equity.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**account_id** | **String** | The account to which the order will clear. | [required] |
**single_order_submission_request** | [**Vec<models::SingleOrderSubmissionRequest>**](singleOrderSubmissionRequest.md) |  | [required] |

### Return type

[**models::OrderPreview**](orderPreview.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_order_status_order_id_get

> models::IserverAccountOrderStatusOrderIdGet200Response iserver_account_order_status_order_id_get(order_id)
Retrieve the status of a single order.

Retrieve the status of a single order.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**order_id** | **String** |  | [required] |

### Return type

[**models::IserverAccountOrderStatusOrderIdGet200Response**](_iserver_account_order_status__orderId__get_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/json; charset=utf-8, text/plain; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_orders_get

> models::LiveOrdersResponse iserver_account_orders_get(filters, force, account_id)
Retrieves open orders and filled or cancelled orders submitted during the current brokerage session.

Retrieves open orders and filled or cancelled orders submitted during the current brokerage session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**filters** | Option<**String**> | Filter results using a comma-separated list of Order Status values. Also accepts a value to sort results by time. |  |
**force** | Option<**bool**> | Instructs IB to clear cache of orders and obtain updated view from brokerage backend. Response will be an empty array. |  |
**account_id** | Option<**String**> | Retrieve orders for a specific account in the structure. |  |

### Return type

[**models::LiveOrdersResponse**](liveOrdersResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_account_trades_get

> Vec<models::TradesResponseInner> iserver_account_trades_get(days, account_id)
Retrieve a list of trades.

Retrieve a list of trades, up to a maximum of 7 days prior.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**days** | Option<**String**> | The number of prior days prior to include in response, up to a maximum of 7. If omitted, only the current day's executions will be returned. |  |
**account_id** | Option<**String**> | Filter trades by account ID or allocation group. |  |

### Return type

[**Vec<models::TradesResponseInner>**](tradesResponse_inner.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_notification_post

> String iserver_notification_post(iserver_notification_post_request)
Respond to a server prompt.

Respond to a server prompt received via ntf webscoket message.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iserver_notification_post_request** | [**IserverNotificationPostRequest**](IserverNotificationPostRequest.md) |  | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_questions_suppress_post

> models::IserverQuestionsSuppressPost200Response iserver_questions_suppress_post(iserver_questions_suppress_post_request)
Suppress the specified order reply messages.

Suppress the specified order reply messages for the duration of the brokerage session.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**iserver_questions_suppress_post_request** | [**IserverQuestionsSuppressPostRequest**](IserverQuestionsSuppressPostRequest.md) |  | [required] |

### Return type

[**models::IserverQuestionsSuppressPost200Response**](_iserver_questions_suppress_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_questions_suppress_reset_post

> models::IserverQuestionsSuppressResetPost200Response iserver_questions_suppress_reset_post()
Removes suppression of all order reply messages.

Removes suppression of all order reply messages that were previously suppressed in the current brokerage session.

### Parameters

This endpoint does not need any parameter.

### Return type

[**models::IserverQuestionsSuppressResetPost200Response**](_iserver_questions_suppress_reset_post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## iserver_reply_reply_id_post

> models::IserverReplyReplyIdPost200Response iserver_reply_reply_id_post(reply_id, iserver_reply_reply_id_post_request)
Confirm an order reply message.

Confirm an order reply message and continue with submission of order ticket.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**reply_id** | **String** | The UUID of the reply message to be confirmed, obtained from an order submission response. | [required] |
**iserver_reply_reply_id_post_request** | [**IserverReplyReplyIdPostRequest**](IserverReplyReplyIdPostRequest.md) |  | [required] |

### Return type

[**models::IserverReplyReplyIdPost200Response**](_iserver_reply__replyId__post_200_response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json, text/plain; charset=utf-8, application/json; charset=utf-8

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

