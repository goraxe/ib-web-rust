# OrderStatus

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**sub_type** | Option<**String**> | Internal use only. | [optional]
**request_id** | Option<**String**> | Internal use only. IB-assigned identifier for the status request. | [optional]
**server_id** | Option<**String**> | IB-assigned meta-identifier used to associate rejected and resubmitted orders following Server Prompts. | [optional]
**order_id** | Option<**i32**> | The IB-assigned order identifier of the order, as provided in the request path. | [optional]
**conidex** | Option<**String**> | Contract ID and routing destination in format 123456@EXCHANGE. | [optional]
**conid** | Option<**String**> | Contract ID of the order's instrument. | [optional]
**symbol** | Option<**String**> | Symbol of the order ticket's instrument. | [optional]
**side** | Option<**String**> | Side of the order ticket. | [optional]
**contract_description_1** | Option<**String**> | Human-readable description of the order's instrument. | [optional]
**listing_exchange** | Option<**String**> | Primary listing exchange of the order ticket's instrument. | [optional]
**option_acct** | Option<**String**> | Internal use only. | [optional]
**company_name** | Option<**String**> | Name of the company or asset associated with the instrument. | [optional]
**size** | Option<**String**> | Remaining unfilled size of the order ticket. Will reflect 0.0 if order is filled in full, cancelled, or otherwise resolved and no longer working. | [optional]
**total_size** | Option<**String**> | The total size of the order ticket. | [optional]
**currency** | Option<**String**> | The currency in which the instrument trades and executions are conducted. | [optional]
**account** | Option<**String**> | The account receiving executions against this order ticket. | [optional]
**order_type** | Option<**String**> | The order's  IB order type. | [optional]
**cum_fill** | Option<**String**> | Cumulative filled quantity of the instrument against the order ticket. | [optional]
**status** | Option<**String**> | Status of the order ticket. | [optional]
**order_ccp_status** | Option<**String**> | IB internal order status. | [optional]
**order_status_description** | Option<**String**> | Human-readable rendering of the order's status meant for presentation in UI. | [optional]
**tif** | Option<**String**> | Time in force of the order ticket. | [optional]
**fg_color** | Option<**String**> | Internal use. IB's UI foreground color in hex. | [optional]
**bg_color** | Option<**String**> | Internal use. IB's UI background color in hex. | [optional]
**order_not_editable** | Option<**bool**> | Indicates whether the order ticket can be modified. | [optional]
**editable_fields** | Option<**String**> | Indicates which fields of the order ticket can be modified currently. | [optional]
**cannot_cancel_order** | Option<**bool**> | Indicates whether the order ticket can be cancelled. | [optional]
**deactivate_order** | Option<**bool**> | Indicates whether the order ticket can be deactivated. | [optional]
**sec_type** | Option<**String**> | IB asset class identifier. | [optional]
**available_chart_periods** | Option<**String**> | Internal use. Indicates chart periods available for the instrument. | [optional]
**order_description** | Option<**String**> | Human-readable description of the status or current result of the order ticket, meant for UI presentation. | [optional]
**order_description_with_contract** | Option<**String**> | Human-readable description of the status or current result of the order ticket, meant for UI presentation. Includes instrument name. | [optional]
**alert_active** | Option<**i32**> | Indicates that an alert is active for the order ticket. | [optional]
**child_order_type** | Option<**String**> | Indicates if the order ticket is hedged, and if so, in what way. 0 = No hedge, A = Attached child hedge order, B = Beta/portfolio hedge | [optional]
**order_clearing_account** | Option<**String**> | The IB account to which the order ticket clears. | [optional]
**size_and_fills** | Option<**String**> | A string reflecting the cumulative fills and total size of the order. | [optional]
**exit_strategy_display_price** | Option<**String**> | Internal use. The UI-displayed price associated with a Client Portal exist strategy. | [optional]
**exit_strategy_chart_description** | Option<**String**> | Internal use. A string describing an active Client Portal exit strategy, or the result of its execution. | [optional]
**average_price** | Option<**String**> | Average price of fills against the order, if any. | [optional]
**exit_strategy_tool_availability** | Option<**String**> | Internal use. Indicates the availability of Client Portal exit strategy tool for the order. | [optional]
**allowed_duplicate_opposite** | Option<**bool**> | Indicates whether an identical order on the opposite side can be placed. | [optional]
**order_time** | Option<**String**> | Time of the order's submission in format YYMMDDhhmmss. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


