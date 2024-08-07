# Account

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**investment_objectives** | Option<**Vec<String>**> |  | [optional]
**brokerage_service_codes** | Option<**Vec<String>**> |  | [optional]
**capabilities** | Option<**Vec<String>**> |  | [optional]
**trading_permissions** | Option<[**Vec<models::TradingPermission>**](TradingPermission.md)> |  | [optional]
**commission_configs** | Option<[**Vec<models::CommissionConfig>**](CommissionConfig.md)> |  | [optional]
**all_exchange_access** | Option<[**Vec<models::ExchangeAccess>**](ExchangeAccess.md)> |  | [optional]
**dvp_instructions** | Option<[**Vec<models::DvpInstruction>**](DVPInstruction.md)> |  | [optional]
**trading_limits** | Option<[**models::TradingLimits**](TradingLimits.md)> |  | [optional]
**advisor_wrap_fees** | Option<[**models::AdvisorWrapFeesType**](AdvisorWrapFeesType.md)> |  | [optional]
**fees_template_name** | Option<**String**> |  | [optional]
**client_commission_schedule** | Option<[**models::CommissionScheduleType**](CommissionScheduleType.md)> |  | [optional]
**client_interest_markup_schedules** | Option<[**Vec<models::InterestMarkupType>**](InterestMarkupType.md)> |  | [optional]
**decendent** | Option<[**models::IraDecedent**](IRADecedent.md)> |  | [optional]
**ira_beneficiaries** | Option<[**models::IraBeneficiariesType**](IRABeneficiariesType.md)> |  | [optional]
**ext_positions_transfers** | Option<[**Vec<models::ExtPositionsTransferType>**](ExtPositionsTransferType.md)> |  | [optional]
**deposit_notification** | Option<[**models::DepositNotification**](DepositNotification.md)> |  | [optional]
**ach_instructions** | Option<[**Vec<models::AchInstruction>**](ACHInstruction.md)> |  | [optional]
**recurring_transactions** | Option<[**Vec<models::RecurringTransaction>**](RecurringTransaction.md)> |  | [optional]
**custodian** | Option<[**models::CustodianType**](CustodianType.md)> |  | [optional]
**successor_custodian** | Option<[**models::CustodianType**](CustodianType.md)> |  | [optional]
**account_rep** | Option<[**models::AccountRep**](AccountRep.md)> |  | [optional]
**id** | Option<**String**> |  | [optional]
**external_id** | Option<**String**> |  | [optional]
**property_profile** | Option<**String**> |  | [optional]
**base_currency** | Option<**String**> |  | [optional]
**employee_plan** | Option<**String**> |  | [optional]
**multi_currency** | Option<**bool**> |  | [optional]
**migration** | Option<**bool**> |  | [optional]
**source_account_id** | Option<**String**> |  | [optional]
**margin** | Option<**String**> |  | [optional]
**ira** | Option<**bool**> |  | [optional]
**ira_type** | Option<**String**> |  | [optional]
**ira_official_title** | Option<**String**> |  | [optional]
**client_active_trading** | Option<**bool**> |  | [optional]
**duplicate** | Option<**bool**> |  | [optional]
**number_of_duplicates** | Option<**i32**> |  | [optional]
**stock_yield_program** | Option<**bool**> |  | [optional]
**alias** | Option<**String**> |  | [optional]
**account_type** | Option<**String**> |  | [optional]
**drip** | Option<**bool**> |  | [optional]
**limited_options** | Option<**bool**> |  | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


