/*
 * IB REST API
 *
 * The IB REST API reference documentation
 *
 * The version of the OpenAPI document: 2.9.0
 * Contact: api@interactivebrokers.com
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Account {
    #[serde(rename = "investmentObjectives", skip_serializing_if = "Option::is_none")]
    pub investment_objectives: Option<Vec<InvestmentObjectives>>,
    #[serde(rename = "brokerageServiceCodes", skip_serializing_if = "Option::is_none")]
    pub brokerage_service_codes: Option<Vec<BrokerageServiceCodes>>,
    #[serde(rename = "capabilities", skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<Vec<Capabilities>>,
    #[serde(rename = "tradingPermissions", skip_serializing_if = "Option::is_none")]
    pub trading_permissions: Option<Vec<models::TradingPermission>>,
    #[serde(rename = "commissionConfigs", skip_serializing_if = "Option::is_none")]
    pub commission_configs: Option<Vec<models::CommissionConfig>>,
    #[serde(rename = "allExchangeAccess", skip_serializing_if = "Option::is_none")]
    pub all_exchange_access: Option<Vec<models::ExchangeAccess>>,
    #[serde(rename = "dvpInstructions", skip_serializing_if = "Option::is_none")]
    pub dvp_instructions: Option<Vec<models::DvpInstruction>>,
    #[serde(rename = "tradingLimits", skip_serializing_if = "Option::is_none")]
    pub trading_limits: Option<Box<models::TradingLimits>>,
    #[serde(rename = "advisorWrapFees", skip_serializing_if = "Option::is_none")]
    pub advisor_wrap_fees: Option<Box<models::AdvisorWrapFeesType>>,
    #[serde(rename = "feesTemplateName", skip_serializing_if = "Option::is_none")]
    pub fees_template_name: Option<String>,
    #[serde(rename = "clientCommissionSchedule", skip_serializing_if = "Option::is_none")]
    pub client_commission_schedule: Option<Box<models::CommissionScheduleType>>,
    #[serde(rename = "clientInterestMarkupSchedules", skip_serializing_if = "Option::is_none")]
    pub client_interest_markup_schedules: Option<Vec<models::InterestMarkupType>>,
    #[serde(rename = "decendent", skip_serializing_if = "Option::is_none")]
    pub decendent: Option<Box<models::IraDecedent>>,
    #[serde(rename = "iraBeneficiaries", skip_serializing_if = "Option::is_none")]
    pub ira_beneficiaries: Option<Box<models::IraBeneficiariesType>>,
    #[serde(rename = "extPositionsTransfers", skip_serializing_if = "Option::is_none")]
    pub ext_positions_transfers: Option<Vec<models::ExtPositionsTransferType>>,
    #[serde(rename = "depositNotification", skip_serializing_if = "Option::is_none")]
    pub deposit_notification: Option<Box<models::DepositNotification>>,
    #[serde(rename = "achInstructions", skip_serializing_if = "Option::is_none")]
    pub ach_instructions: Option<Vec<models::AchInstruction>>,
    #[serde(rename = "recurringTransactions", skip_serializing_if = "Option::is_none")]
    pub recurring_transactions: Option<Vec<models::RecurringTransaction>>,
    #[serde(rename = "custodian", skip_serializing_if = "Option::is_none")]
    pub custodian: Option<Box<models::CustodianType>>,
    #[serde(rename = "successorCustodian", skip_serializing_if = "Option::is_none")]
    pub successor_custodian: Option<Box<models::CustodianType>>,
    #[serde(rename = "accountRep", skip_serializing_if = "Option::is_none")]
    pub account_rep: Option<Box<models::AccountRep>>,
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "externalId", skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "propertyProfile", skip_serializing_if = "Option::is_none")]
    pub property_profile: Option<String>,
    #[serde(rename = "baseCurrency", skip_serializing_if = "Option::is_none")]
    pub base_currency: Option<BaseCurrency>,
    #[serde(rename = "employeePlan", skip_serializing_if = "Option::is_none")]
    pub employee_plan: Option<String>,
    #[serde(rename = "multiCurrency", skip_serializing_if = "Option::is_none")]
    pub multi_currency: Option<bool>,
    #[serde(rename = "migration", skip_serializing_if = "Option::is_none")]
    pub migration: Option<bool>,
    #[serde(rename = "sourceAccountId", skip_serializing_if = "Option::is_none")]
    pub source_account_id: Option<String>,
    #[serde(rename = "margin", skip_serializing_if = "Option::is_none")]
    pub margin: Option<String>,
    #[serde(rename = "ira", skip_serializing_if = "Option::is_none")]
    pub ira: Option<bool>,
    #[serde(rename = "iraType", skip_serializing_if = "Option::is_none")]
    pub ira_type: Option<IraType>,
    #[serde(rename = "iraOfficialTitle", skip_serializing_if = "Option::is_none")]
    pub ira_official_title: Option<String>,
    #[serde(rename = "clientActiveTrading", skip_serializing_if = "Option::is_none")]
    pub client_active_trading: Option<bool>,
    #[serde(rename = "duplicate", skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<bool>,
    #[serde(rename = "numberOfDuplicates", skip_serializing_if = "Option::is_none")]
    pub number_of_duplicates: Option<i32>,
    #[serde(rename = "stockYieldProgram", skip_serializing_if = "Option::is_none")]
    pub stock_yield_program: Option<bool>,
    #[serde(rename = "alias", skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(rename = "accountType", skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    #[serde(rename = "drip", skip_serializing_if = "Option::is_none")]
    pub drip: Option<bool>,
    #[serde(rename = "limitedOptions", skip_serializing_if = "Option::is_none")]
    pub limited_options: Option<bool>,
}

impl Account {
    pub fn new() -> Account {
        Account {
            investment_objectives: None,
            brokerage_service_codes: None,
            capabilities: None,
            trading_permissions: None,
            commission_configs: None,
            all_exchange_access: None,
            dvp_instructions: None,
            trading_limits: None,
            advisor_wrap_fees: None,
            fees_template_name: None,
            client_commission_schedule: None,
            client_interest_markup_schedules: None,
            decendent: None,
            ira_beneficiaries: None,
            ext_positions_transfers: None,
            deposit_notification: None,
            ach_instructions: None,
            recurring_transactions: None,
            custodian: None,
            successor_custodian: None,
            account_rep: None,
            id: None,
            external_id: None,
            property_profile: None,
            base_currency: None,
            employee_plan: None,
            multi_currency: None,
            migration: None,
            source_account_id: None,
            margin: None,
            ira: None,
            ira_type: None,
            ira_official_title: None,
            client_active_trading: None,
            duplicate: None,
            number_of_duplicates: None,
            stock_yield_program: None,
            alias: None,
            account_type: None,
            drip: None,
            limited_options: None,
        }
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum InvestmentObjectives {
    #[serde(rename = "Trading")]
    Trading,
    #[serde(rename = "Growth")]
    Growth,
    #[serde(rename = "Speculation")]
    Speculation,
    #[serde(rename = "Hedging")]
    Hedging,
    #[serde(rename = "Preservation")]
    Preservation,
    #[serde(rename = "Income")]
    Income,
}

impl Default for InvestmentObjectives {
    fn default() -> InvestmentObjectives {
        Self::Trading
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BrokerageServiceCodes {
    #[serde(rename = "IBClearing")]
    IbClearing,
    #[serde(rename = "IBExecution")]
    IbExecution,
    #[serde(rename = "IBPrime")]
    IbPrime,
}

impl Default for BrokerageServiceCodes {
    fn default() -> BrokerageServiceCodes {
        Self::IbClearing
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Capabilities {
    #[serde(rename = "BOND")]
    Bond,
    #[serde(rename = "FOP")]
    Fop,
    #[serde(rename = "FUND")]
    Fund,
    #[serde(rename = "FUT")]
    Fut,
    #[serde(rename = "MRGN")]
    Mrgn,
    #[serde(rename = "MULT")]
    Mult,
    #[serde(rename = "OPT")]
    Opt,
    #[serde(rename = "SSF")]
    Ssf,
    #[serde(rename = "CFD")]
    Cfd,
    #[serde(rename = "STK")]
    Stk,
    #[serde(rename = "CLP")]
    Clp,
    #[serde(rename = "LEVFX")]
    Levfx,
    #[serde(rename = "CMDTY")]
    Cmdty,
}

impl Default for Capabilities {
    fn default() -> Capabilities {
        Self::Bond
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BaseCurrency {
    #[serde(rename = "USD")]
    Usd,
    #[serde(rename = "EUR")]
    Eur,
    #[serde(rename = "GBP")]
    Gbp,
    #[serde(rename = "CAD")]
    Cad,
    #[serde(rename = "JPY")]
    Jpy,
    #[serde(rename = "HKD")]
    Hkd,
    #[serde(rename = "AUD")]
    Aud,
    #[serde(rename = "CHF")]
    Chf,
    #[serde(rename = "MXN")]
    Mxn,
    #[serde(rename = "SEK")]
    Sek,
    #[serde(rename = "NZD")]
    Nzd,
    #[serde(rename = "HUF")]
    Huf,
    #[serde(rename = "CZK")]
    Czk,
    #[serde(rename = "CNH")]
    Cnh,
    #[serde(rename = "DKK")]
    Dkk,
    #[serde(rename = "RUB")]
    Rub,
    #[serde(rename = "ILS")]
    Ils,
    #[serde(rename = "NOK")]
    Nok,
    #[serde(rename = "SGD")]
    Sgd,
    #[serde(rename = "PLN")]
    Pln,
    #[serde(rename = "ZAR")]
    Zar,
    #[serde(rename = "AED")]
    Aed,
    #[serde(rename = "KRW")]
    Krw,
    #[serde(rename = "SAR")]
    Sar,
    #[serde(rename = "TRY")]
    Try,
}

impl Default for BaseCurrency {
    fn default() -> BaseCurrency {
        Self::Usd
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IraType {
    #[serde(rename = "RI")]
    Ri,
    #[serde(rename = "RO")]
    Ro,
    #[serde(rename = "RT")]
    Rt,
    #[serde(rename = "SP")]
    Sp,
    #[serde(rename = "ED")]
    Ed,
    #[serde(rename = "TH")]
    Th,
    #[serde(rename = "RH")]
    Rh,
    #[serde(rename = "SH")]
    Sh,
    #[serde(rename = "RRSP")]
    Rrsp,
    #[serde(rename = "SRRSP")]
    Srrsp,
    #[serde(rename = "TFSA")]
    Tfsa,
    #[serde(rename = "SIMPLE")]
    Simple,
    #[serde(rename = "ISA")]
    Isa,
}

impl Default for IraType {
    fn default() -> IraType {
        Self::Ri
    }
}
/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum AccountType {
    #[serde(rename = "Investment")]
    Investment,
    #[serde(rename = "Trading")]
    Trading,
    #[serde(rename = "SMSF")]
    Smsf,
}

impl Default for AccountType {
    fn default() -> AccountType {
        Self::Investment
    }
}

