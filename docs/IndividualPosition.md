# IndividualPosition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**acct_id** | Option<**String**> | IB accountId of an account with a position in the requested conid. | [optional]
**all_exchanges** | Option<**String**> | Comma separated all exchanges on which the instrument trades. | [optional]
**asset_class** | Option<**String**> | Asset class of the requested instrument. | [optional]
**avg_cost** | Option<**f64**> | The account's average cost for its position. | [optional]
**avg_price** | Option<**f64**> | The account's average price for its position. | [optional]
**base_avg_cost** | Option<**f64**> | Average cost in the account's base currency. | [optional]
**base_avg_price** | Option<**f64**> | Average price in the account's base currency. | [optional]
**base_mkt_price** | Option<**f64**> | Market price of instrument in the account's base currency. | [optional]
**base_mkt_value** | Option<**f64**> | Market value of the position in the account's base currency. | [optional]
**base_realized_pnl** | Option<**f64**> | Realized PnL for the instrument in the account's base currency. | [optional]
**base_unrealized_pnl** | Option<**f64**> | Unrealized PnL for the instrument in the account's base currency. | [optional]
**chinese_name** | Option<**String**> | Chinese name of the instrument. | [optional]
**con_exch_map** | Option<**Vec<String>**> |  | [optional]
**conid** | Option<**i32**> | IB contract ID for the instrument. | [optional]
**contract_desc** | Option<**String**> | Human-readable description of the instrument. | [optional]
**country_code** | Option<**String**> | Country in which the instrument is issued. | [optional]
**currency** | Option<**String**> | Currency in which the instrument trades. | [optional]
**display_rule** | Option<[**models::IndividualPositionDisplayRule**](individualPosition_displayRule.md)> |  | [optional]
**exchs** | Option<[**serde_json::Value**](.md)> |  | [optional]
**exercise_style** | Option<**String**> | Style of exercise for options. | [optional]
**expiry** | Option<**String**> | Expiration of instrument, if applicable. | [optional]
**full_name** | Option<**String**> | Full display name of the instrument. | [optional]
**group** | Option<**String**> | Industry sub-categorization of the instrument. | [optional]
**has_options** | Option<**bool**> | Indicates whether instrument has options contracts available for trading at IB. | [optional]
**increment_rules** | Option<[**Vec<models::IndividualPositionIncrementRulesInner>**](individualPosition_incrementRules_inner.md)> | Array containing increment rules used when pricing orders for the instrument. | [optional]
**is_event_contract** | Option<**bool**> | Indicates whether the instrument is an Event Contract. | [optional]
**is_us** | Option<**bool**> | Indicates whether the instrument is issued in the US. | [optional]
**last_trading_day** | Option<**String**> | Last day of trading in the instrument, if applicable. Formatted `YYYYMMDD`. | [optional]
**listing_exchange** | Option<**String**> | The exchange on which the instrument is listed, or the primary exchange recognized by IB for the instrument. | [optional]
**mkt_price** | Option<**f64**> | Current market price of the instrument, in the instrument's currency. | [optional]
**mkt_value** | Option<**f64**> | Current market value of the account's position in the instrument, in the instrument's currency. | [optional]
**model** | Option<**String**> | Name of the model portfolio in which the account is invested that contributes this position. | [optional]
**multiplier** | Option<**f64**> | Instrument's multiplier, if applicable. | [optional]
**name** | Option<**String**> | Formal name of the entity or asset to which the instrument relates. | [optional]
**page_size** | Option<**i32**> | Maximum number of accounts that can be returned in a single request. | [optional]
**position** | Option<**f64**> | Size of position in units of instrument. | [optional]
**put_or_call** | Option<**String**> | The right of an options contract, if applicable. | [optional]
**realized_pnl** | Option<**f64**> | Realized PnL for the instrument in the instrument's currency. | [optional]
**sector** | Option<**String**> | Industry sector categorization of the instrument. | [optional]
**sector_group** | Option<**String**> | Industry sub-categorization of the instrument. | [optional]
**strike** | Option<**String**> | Strike price, if applicable. Returned as string. | [optional]
**ticker** | Option<**String**> | Symbol associated with the instrument. | [optional]
**time** | Option<**i32**> | Time taken to retrieve position data in milliseconds. | [optional]
**r#type** | Option<**String**> | Description of instrument, used to differentiate classes, if applicable. | [optional]
**und_conid** | Option<**i32**> | Contract ID of underlying instrument, if applicable. | [optional]
**unrealized_pnl** | Option<**f64**> | Unrealized PnL for the instrument in the account. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


