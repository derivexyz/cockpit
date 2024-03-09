#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///BalanceUpdateSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "name",
    "new_balance",
    "previous_balance",
    "update_type"
  ],
  "properties": {
    "name": {
      "title": "name",
      "description": "Name of colletaral asset or instrument",
      "type": "string"
    },
    "new_balance": {
      "title": "new_balance",
      "description": "Balance after update",
      "type": "string",
      "format": "decimal"
    },
    "previous_balance": {
      "title": "previous_balance",
      "description": "Balance before update",
      "type": "string",
      "format": "decimal"
    },
    "update_type": {
      "title": "update_type",
      "description": "Type of transaction",
      "type": "string",
      "enum": [
        "trade",
        "asset_deposit",
        "asset_withdrawal",
        "transfer",
        "subaccount_deposit",
        "subaccount_withdrawal",
        "liquidation",
        "onchain_drift_fix",
        "perp_settlement",
        "option_settlement",
        "interest_accrual",
        "onchain_revert"
      ]
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct BalanceUpdateSchema {
    ///Name of colletaral asset or instrument
    pub name: String,
    ///Balance after update
    pub new_balance: bigdecimal::BigDecimal,
    ///Balance before update
    pub previous_balance: bigdecimal::BigDecimal,
    ///Type of transaction
    pub update_type: UpdateType,
}
impl From<&BalanceUpdateSchema> for BalanceUpdateSchema {
    fn from(value: &BalanceUpdateSchema) -> Self {
        value.clone()
    }
}
/**Subscribe to changes in user's positions for a given subaccount ID.

For perpetuals, additional balance updates are emitted under the name Q-{ccy}-PERP where Q stands for "quote".
This balance is a proxy for an on-chain state of lastMarkPrice.
Because of a synchronization lag with the on-chain state, the orderbook instead keeps track of a running total cost of perpetual trades,

For example:
Q-ETH-PERP balance of $6,600 and an ETH-PERP balance of 3 means the lastMarkPrice state is estimated to be $2,200.*/
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "{subaccount_id}.balances",
  "description": "Subscribe to changes in user's positions for a given subaccount ID.\n\nFor perpetuals, additional balance updates are emitted under the name Q-{ccy}-PERP where Q stands for \"quote\".\nThis balance is a proxy for an on-chain state of lastMarkPrice.\nBecause of a synchronization lag with the on-chain state, the orderbook instead keeps track of a running total cost of perpetual trades,\n\nFor example:\nQ-ETH-PERP balance of $6,600 and an ETH-PERP balance of 3 means the lastMarkPrice state is estimated to be $2,200.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/SubaccountIdBalancesPubSubSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SubaccountIdBalances(pub SubaccountIdBalancesPubSubSchema);
impl std::ops::Deref for SubaccountIdBalances {
    type Target = SubaccountIdBalancesPubSubSchema;
    fn deref(&self) -> &SubaccountIdBalancesPubSubSchema {
        &self.0
    }
}
impl From<SubaccountIdBalances> for SubaccountIdBalancesPubSubSchema {
    fn from(value: SubaccountIdBalances) -> Self {
        value.0
    }
}
impl From<&SubaccountIdBalances> for SubaccountIdBalances {
    fn from(value: &SubaccountIdBalances) -> Self {
        value.clone()
    }
}
impl From<SubaccountIdBalancesPubSubSchema> for SubaccountIdBalances {
    fn from(value: SubaccountIdBalancesPubSubSchema) -> Self {
        Self(value)
    }
}
///SubaccountIdBalancesChannelSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "subaccount_id"
  ],
  "properties": {
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount ID",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdBalancesChannelSchema {
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&SubaccountIdBalancesChannelSchema> for SubaccountIdBalancesChannelSchema {
    fn from(value: &SubaccountIdBalancesChannelSchema) -> Self {
        value.clone()
    }
}
///SubaccountIdBalancesNotificationParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "channel",
    "data"
  ],
  "properties": {
    "channel": {
      "title": "channel",
      "description": "Subscribed channel name",
      "type": "string"
    },
    "data": {
      "title": "data",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/BalanceUpdateSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdBalancesNotificationParamsSchema {
    ///Subscribed channel name
    pub channel: String,
    pub data: Vec<BalanceUpdateSchema>,
}
impl From<&SubaccountIdBalancesNotificationParamsSchema>
for SubaccountIdBalancesNotificationParamsSchema {
    fn from(value: &SubaccountIdBalancesNotificationParamsSchema) -> Self {
        value.clone()
    }
}
///SubaccountIdBalancesNotificationSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "method",
    "params"
  ],
  "properties": {
    "method": {
      "title": "method",
      "type": "string"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/SubaccountIdBalancesNotificationParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdBalancesNotificationSchema {
    pub method: String,
    pub params: SubaccountIdBalancesNotificationParamsSchema,
}
impl From<&SubaccountIdBalancesNotificationSchema>
for SubaccountIdBalancesNotificationSchema {
    fn from(value: &SubaccountIdBalancesNotificationSchema) -> Self {
        value.clone()
    }
}
///SubaccountIdBalancesPubSubSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "channel_params",
    "notification"
  ],
  "properties": {
    "channel_params": {
      "type": "object",
      "$ref": "#/definitions/SubaccountIdBalancesChannelSchema",
      "field_many": false
    },
    "notification": {
      "type": "object",
      "$ref": "#/definitions/SubaccountIdBalancesNotificationSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct SubaccountIdBalancesPubSubSchema {
    pub channel_params: SubaccountIdBalancesChannelSchema,
    pub notification: SubaccountIdBalancesNotificationSchema,
}
impl From<&SubaccountIdBalancesPubSubSchema> for SubaccountIdBalancesPubSubSchema {
    fn from(value: &SubaccountIdBalancesPubSubSchema) -> Self {
        value.clone()
    }
}
///Type of transaction
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "update_type",
  "description": "Type of transaction",
  "type": "string",
  "enum": [
    "trade",
    "asset_deposit",
    "asset_withdrawal",
    "transfer",
    "subaccount_deposit",
    "subaccount_withdrawal",
    "liquidation",
    "onchain_drift_fix",
    "perp_settlement",
    "option_settlement",
    "interest_accrual",
    "onchain_revert"
  ]
}*/
/// ```
/// </details>
#[derive(
    Clone,
    Copy,
    Debug,
    Deserialize,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize
)]
pub enum UpdateType {
    #[serde(rename = "trade")]
    Trade,
    #[serde(rename = "asset_deposit")]
    AssetDeposit,
    #[serde(rename = "asset_withdrawal")]
    AssetWithdrawal,
    #[serde(rename = "transfer")]
    Transfer,
    #[serde(rename = "subaccount_deposit")]
    SubaccountDeposit,
    #[serde(rename = "subaccount_withdrawal")]
    SubaccountWithdrawal,
    #[serde(rename = "liquidation")]
    Liquidation,
    #[serde(rename = "onchain_drift_fix")]
    OnchainDriftFix,
    #[serde(rename = "perp_settlement")]
    PerpSettlement,
    #[serde(rename = "option_settlement")]
    OptionSettlement,
    #[serde(rename = "interest_accrual")]
    InterestAccrual,
    #[serde(rename = "onchain_revert")]
    OnchainRevert,
}
impl From<&UpdateType> for UpdateType {
    fn from(value: &UpdateType) -> Self {
        value.clone()
    }
}
impl ToString for UpdateType {
    fn to_string(&self) -> String {
        match *self {
            Self::Trade => "trade".to_string(),
            Self::AssetDeposit => "asset_deposit".to_string(),
            Self::AssetWithdrawal => "asset_withdrawal".to_string(),
            Self::Transfer => "transfer".to_string(),
            Self::SubaccountDeposit => "subaccount_deposit".to_string(),
            Self::SubaccountWithdrawal => "subaccount_withdrawal".to_string(),
            Self::Liquidation => "liquidation".to_string(),
            Self::OnchainDriftFix => "onchain_drift_fix".to_string(),
            Self::PerpSettlement => "perp_settlement".to_string(),
            Self::OptionSettlement => "option_settlement".to_string(),
            Self::InterestAccrual => "interest_accrual".to_string(),
            Self::OnchainRevert => "onchain_revert".to_string(),
        }
    }
}
impl std::str::FromStr for UpdateType {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        match value {
            "trade" => Ok(Self::Trade),
            "asset_deposit" => Ok(Self::AssetDeposit),
            "asset_withdrawal" => Ok(Self::AssetWithdrawal),
            "transfer" => Ok(Self::Transfer),
            "subaccount_deposit" => Ok(Self::SubaccountDeposit),
            "subaccount_withdrawal" => Ok(Self::SubaccountWithdrawal),
            "liquidation" => Ok(Self::Liquidation),
            "onchain_drift_fix" => Ok(Self::OnchainDriftFix),
            "perp_settlement" => Ok(Self::PerpSettlement),
            "option_settlement" => Ok(Self::OptionSettlement),
            "interest_accrual" => Ok(Self::InterestAccrual),
            "onchain_revert" => Ok(Self::OnchainRevert),
            _ => Err("invalid value"),
        }
    }
}
impl std::convert::TryFrom<&str> for UpdateType {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for UpdateType {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for UpdateType {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
