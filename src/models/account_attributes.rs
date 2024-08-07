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
pub struct AccountAttributes {
    /// User-defined alias assigned to the account for easy identification.
    #[serde(rename = "accountAlias", default, with = "::serde_with::rust::double_option", skip_serializing_if = "Option::is_none")]
    pub account_alias: Option<Option<String>>,
    /// Unix epoch timestamp of account opening.
    #[serde(rename = "accountStatus", skip_serializing_if = "Option::is_none")]
    pub account_status: Option<i32>,
    /// A name assigned to the account, typically the account holder name or business entity.
    #[serde(rename = "accountTitle", skip_serializing_if = "Option::is_none")]
    pub account_title: Option<String>,
    /// The account's virtual account number, or otherwise its IB accountId if no VAN is set.
    #[serde(rename = "accountVan", skip_serializing_if = "Option::is_none")]
    pub account_van: Option<String>,
    /// Identifies the type of client with which the account is associated, such as an individual or LLC.
    #[serde(rename = "acctCustType", skip_serializing_if = "Option::is_none")]
    pub acct_cust_type: Option<String>,
    /// Indicates whether account can receive live orders (do not mix with paper trading).
    #[serde(rename = "brokerageAccess", skip_serializing_if = "Option::is_none")]
    pub brokerage_access: Option<bool>,
    /// A descriptor of the nature of the account, reflecting the responsible group within IB.
    #[serde(rename = "businessType", skip_serializing_if = "Option::is_none")]
    pub business_type: Option<BusinessType>,
    /// Status of the account with respect to clearing at IB. O is open, P pending, N new, A abandoned, C closed, R rejected.
    #[serde(rename = "clearingStatus", skip_serializing_if = "Option::is_none")]
    pub clearing_status: Option<ClearingStatus>,
    /// Indicates a Covestor account.
    #[serde(rename = "covestor", skip_serializing_if = "Option::is_none")]
    pub covestor: Option<bool>,
    /// Base currency of the account.
    #[serde(rename = "currency", skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Internal human-readable description of the account.
    #[serde(rename = "desc", skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    /// Displayed name of the account in UI. Will reflect either the accountId or accountAlias, if set.
    #[serde(rename = "displayName", skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    /// Indicates that the account is managed by a financial advisor.
    #[serde(rename = "faClient", skip_serializing_if = "Option::is_none")]
    pub fa_client: Option<bool>,
    /// IB business entity under which the account resides.
    #[serde(rename = "ibEntity", skip_serializing_if = "Option::is_none")]
    pub ib_entity: Option<IbEntity>,
    /// The account's IB accountId.
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Indicates that trading by the client is disabled in the account.
    #[serde(rename = "noClientTrading", skip_serializing_if = "Option::is_none")]
    pub no_client_trading: Option<bool>,
    #[serde(rename = "parent", skip_serializing_if = "Option::is_none")]
    pub parent: Option<Box<models::AccountAttributesParent>>,
    /// Indicates whether account has a prepaid crypto segment (Crypto Plus) with PAXOS.
    #[serde(rename = "PrepaidCrypto-P", skip_serializing_if = "Option::is_none")]
    pub prepaid_crypto_p: Option<bool>,
    /// Indicates whether account has a prepaid crypto segment (Crypto Plus) with ZEROHASH.
    #[serde(rename = "PrepaidCrypto-Z", skip_serializing_if = "Option::is_none")]
    pub prepaid_crypto_z: Option<bool>,
    /// Indicates that virtual forex positions are tracked in the account.
    #[serde(rename = "trackVirtualFXPortfolio", skip_serializing_if = "Option::is_none")]
    pub track_virtual_fx_portfolio: Option<bool>,
    /// Internal identifier used by IB to reflect the trading permissions of the account.
    #[serde(rename = "tradingType", skip_serializing_if = "Option::is_none")]
    pub trading_type: Option<TradingType>,
    /// Indicates whether the account exists in production, paper, or demo environments.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub r#type: Option<Type>,
}

impl AccountAttributes {
    pub fn new() -> AccountAttributes {
        AccountAttributes {
            account_alias: None,
            account_status: None,
            account_title: None,
            account_van: None,
            acct_cust_type: None,
            brokerage_access: None,
            business_type: None,
            clearing_status: None,
            covestor: None,
            currency: None,
            desc: None,
            display_name: None,
            fa_client: None,
            ib_entity: None,
            id: None,
            no_client_trading: None,
            parent: None,
            prepaid_crypto_p: None,
            prepaid_crypto_z: None,
            track_virtual_fx_portfolio: None,
            trading_type: None,
            r#type: None,
        }
    }
}
/// A descriptor of the nature of the account, reflecting the responsible group within IB.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum BusinessType {
    #[serde(rename = "IB_SALES")]
    Sales,
    #[serde(rename = "IB_PROSERVE")]
    Proserve,
}

impl Default for BusinessType {
    fn default() -> BusinessType {
        Self::Sales
    }
}
/// Status of the account with respect to clearing at IB. O is open, P pending, N new, A abandoned, C closed, R rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ClearingStatus {
    #[serde(rename = "O")]
    O,
    #[serde(rename = "P")]
    P,
    #[serde(rename = "N")]
    N,
    #[serde(rename = "A")]
    A,
    #[serde(rename = "R")]
    R,
    #[serde(rename = "C")]
    C,
}

impl Default for ClearingStatus {
    fn default() -> ClearingStatus {
        Self::O
    }
}
/// Base currency of the account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename = "USD")]
    Usd,
}

impl Default for Currency {
    fn default() -> Currency {
        Self::Usd
    }
}
/// IB business entity under which the account resides.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum IbEntity {
    #[serde(rename = "IBLLC-US")]
    IbllcUs,
    #[serde(rename = "IB-CAN")]
    IbCan,
    #[serde(rename = "IB-UK")]
    IbUk,
    #[serde(rename = "IB-IE")]
    IbIe,
}

impl Default for IbEntity {
    fn default() -> IbEntity {
        Self::IbllcUs
    }
}
/// Internal identifier used by IB to reflect the trading permissions of the account.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum TradingType {
    #[serde(rename = "STKNOPT")]
    Stknopt,
}

impl Default for TradingType {
    fn default() -> TradingType {
        Self::Stknopt
    }
}
/// Indicates whether the account exists in production, paper, or demo environments.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "DEMO")]
    Demo,
}

impl Default for Type {
    fn default() -> Type {
        Self::Demo
    }
}

