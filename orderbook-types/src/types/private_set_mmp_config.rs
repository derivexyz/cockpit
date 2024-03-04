#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Set the mmp config for the subaccount and currency
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/set_mmp_config",
  "description": "Set the mmp config for the subaccount and currency",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateSetMmpConfigJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateSetMmpConfig(pub PrivateSetMmpConfigJsonrpcSchema);
impl std::ops::Deref for PrivateSetMmpConfig {
    type Target = PrivateSetMmpConfigJsonrpcSchema;
    fn deref(&self) -> &PrivateSetMmpConfigJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateSetMmpConfig> for PrivateSetMmpConfigJsonrpcSchema {
    fn from(value: PrivateSetMmpConfig) -> Self {
        value.0
    }
}
impl From<&PrivateSetMmpConfig> for PrivateSetMmpConfig {
    fn from(value: &PrivateSetMmpConfig) -> Self {
        value.clone()
    }
}
impl From<PrivateSetMmpConfigJsonrpcSchema> for PrivateSetMmpConfig {
    fn from(value: PrivateSetMmpConfigJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateSetMmpConfigJsonrpcSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "request",
    "response"
  ],
  "properties": {
    "request": {
      "type": "object",
      "$ref": "#/definitions/PrivateSetMmpConfigRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateSetMmpConfigResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetMmpConfigJsonrpcSchema {
    pub request: PrivateSetMmpConfigRequestSchema,
    pub response: PrivateSetMmpConfigResponseSchema,
}
impl From<&PrivateSetMmpConfigJsonrpcSchema> for PrivateSetMmpConfigJsonrpcSchema {
    fn from(value: &PrivateSetMmpConfigJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateSetMmpConfigParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "mmp_frozen_time",
    "mmp_interval",
    "subaccount_id"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency of this mmp config",
      "type": "string"
    },
    "mmp_amount_limit": {
      "title": "mmp_amount_limit",
      "description": "Maximum total order amount that can be traded within the mmp_interval across all instruments of the provided currency. The amounts are not netted, so a filled bid of 1 and a filled ask of 2 would count as 3.<br />Default: 0 (no limit)",
      "default": "0",
      "type": "string",
      "format": "decimal"
    },
    "mmp_delta_limit": {
      "title": "mmp_delta_limit",
      "description": "Maximum total delta that can be traded within the mmp_interval across all instruments of the provided currency. This quantity is netted, so a filled order with +1 delta and a filled order with -2 delta would count as -1<br />Default: 0 (no limit)",
      "default": "0",
      "type": "string",
      "format": "decimal"
    },
    "mmp_frozen_time": {
      "title": "mmp_frozen_time",
      "description": "Time interval in ms setting how long the subaccount is frozen after an mmp trigger, if 0 then a manual reset would be required via private/reset_mmp",
      "type": "integer"
    },
    "mmp_interval": {
      "title": "mmp_interval",
      "description": "Time interval in ms over which the limits are monotored, if 0 then mmp is disabled",
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to set the config",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetMmpConfigParamsSchema {
    ///Currency of this mmp config
    pub currency: String,
    ///Maximum total order amount that can be traded within the mmp_interval across all instruments of the provided currency. The amounts are not netted, so a filled bid of 1 and a filled ask of 2 would count as 3.<br />Default: 0 (no limit)
    #[serde(default = "defaults::private_set_mmp_config_params_schema_mmp_amount_limit")]
    pub mmp_amount_limit: bigdecimal::BigDecimal,
    ///Maximum total delta that can be traded within the mmp_interval across all instruments of the provided currency. This quantity is netted, so a filled order with +1 delta and a filled order with -2 delta would count as -1<br />Default: 0 (no limit)
    #[serde(default = "defaults::private_set_mmp_config_params_schema_mmp_delta_limit")]
    pub mmp_delta_limit: bigdecimal::BigDecimal,
    ///Time interval in ms setting how long the subaccount is frozen after an mmp trigger, if 0 then a manual reset would be required via private/reset_mmp
    pub mmp_frozen_time: i64,
    ///Time interval in ms over which the limits are monotored, if 0 then mmp is disabled
    pub mmp_interval: i64,
    ///Subaccount_id for which to set the config
    pub subaccount_id: i64,
}
impl From<&PrivateSetMmpConfigParamsSchema> for PrivateSetMmpConfigParamsSchema {
    fn from(value: &PrivateSetMmpConfigParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateSetMmpConfigRequestSchema
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
    "id": {
      "oneOf": [
        {
          "title": "",
          "type": "string"
        },
        {
          "title": "",
          "type": "integer"
        }
      ]
    },
    "method": {
      "title": "method",
      "type": "string",
      "const": "private/set_mmp_config"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateSetMmpConfigParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetMmpConfigRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateSetMmpConfigRequestSchemaId>,
    pub method: String,
    pub params: PrivateSetMmpConfigParamsSchema,
}
impl From<&PrivateSetMmpConfigRequestSchema> for PrivateSetMmpConfigRequestSchema {
    fn from(value: &PrivateSetMmpConfigRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateSetMmpConfigRequestSchemaId
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "oneOf": [
    {
      "title": "",
      "type": "string"
    },
    {
      "title": "",
      "type": "integer"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PrivateSetMmpConfigRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSetMmpConfigRequestSchemaId> for PrivateSetMmpConfigRequestSchemaId {
    fn from(value: &PrivateSetMmpConfigRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSetMmpConfigRequestSchemaId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for PrivateSetMmpConfigRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSetMmpConfigRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSetMmpConfigRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSetMmpConfigRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSetMmpConfigRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateSetMmpConfigResponseSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "id",
    "result"
  ],
  "properties": {
    "id": {
      "oneOf": [
        {
          "title": "",
          "type": "string"
        },
        {
          "title": "",
          "type": "integer"
        }
      ]
    },
    "result": {
      "description": "",
      "type": "object",
      "$ref": "#/definitions/PrivateSetMmpConfigResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetMmpConfigResponseSchema {
    pub id: PrivateSetMmpConfigResponseSchemaId,
    ///
    pub result: PrivateSetMmpConfigResultSchema,
}
impl From<&PrivateSetMmpConfigResponseSchema> for PrivateSetMmpConfigResponseSchema {
    fn from(value: &PrivateSetMmpConfigResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateSetMmpConfigResponseSchemaId
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "oneOf": [
    {
      "title": "",
      "type": "string"
    },
    {
      "title": "",
      "type": "integer"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PrivateSetMmpConfigResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateSetMmpConfigResponseSchemaId> for PrivateSetMmpConfigResponseSchemaId {
    fn from(value: &PrivateSetMmpConfigResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateSetMmpConfigResponseSchemaId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::Variant1(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for PrivateSetMmpConfigResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateSetMmpConfigResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateSetMmpConfigResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateSetMmpConfigResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateSetMmpConfigResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateSetMmpConfigResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "currency",
    "mmp_frozen_time",
    "mmp_interval",
    "subaccount_id"
  ],
  "properties": {
    "currency": {
      "title": "currency",
      "description": "Currency of this mmp config",
      "type": "string"
    },
    "mmp_amount_limit": {
      "title": "mmp_amount_limit",
      "description": "Maximum total order amount that can be traded within the mmp_interval across all instruments of the provided currency. The amounts are not netted, so a filled bid of 1 and a filled ask of 2 would count as 3.<br />Default: 0 (no limit)",
      "default": "0",
      "type": "string",
      "format": "decimal"
    },
    "mmp_delta_limit": {
      "title": "mmp_delta_limit",
      "description": "Maximum total delta that can be traded within the mmp_interval across all instruments of the provided currency. This quantity is netted, so a filled order with +1 delta and a filled order with -2 delta would count as -1<br />Default: 0 (no limit)",
      "default": "0",
      "type": "string",
      "format": "decimal"
    },
    "mmp_frozen_time": {
      "title": "mmp_frozen_time",
      "description": "Time interval in ms setting how long the subaccount is frozen after an mmp trigger, if 0 then a manual reset would be required via private/reset_mmp",
      "type": "integer"
    },
    "mmp_interval": {
      "title": "mmp_interval",
      "description": "Time interval in ms over which the limits are monotored, if 0 then mmp is disabled",
      "type": "integer"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to set the config",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateSetMmpConfigResultSchema {
    ///Currency of this mmp config
    pub currency: String,
    ///Maximum total order amount that can be traded within the mmp_interval across all instruments of the provided currency. The amounts are not netted, so a filled bid of 1 and a filled ask of 2 would count as 3.<br />Default: 0 (no limit)
    #[serde(default = "defaults::private_set_mmp_config_result_schema_mmp_amount_limit")]
    pub mmp_amount_limit: bigdecimal::BigDecimal,
    ///Maximum total delta that can be traded within the mmp_interval across all instruments of the provided currency. This quantity is netted, so a filled order with +1 delta and a filled order with -2 delta would count as -1<br />Default: 0 (no limit)
    #[serde(default = "defaults::private_set_mmp_config_result_schema_mmp_delta_limit")]
    pub mmp_delta_limit: bigdecimal::BigDecimal,
    ///Time interval in ms setting how long the subaccount is frozen after an mmp trigger, if 0 then a manual reset would be required via private/reset_mmp
    pub mmp_frozen_time: i64,
    ///Time interval in ms over which the limits are monotored, if 0 then mmp is disabled
    pub mmp_interval: i64,
    ///Subaccount_id for which to set the config
    pub subaccount_id: i64,
}
impl From<&PrivateSetMmpConfigResultSchema> for PrivateSetMmpConfigResultSchema {
    fn from(value: &PrivateSetMmpConfigResultSchema) -> Self {
        value.clone()
    }
}
pub mod defaults {
    pub(super) fn private_set_mmp_config_params_schema_mmp_amount_limit() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn private_set_mmp_config_params_schema_mmp_delta_limit() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn private_set_mmp_config_result_schema_mmp_amount_limit() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn private_set_mmp_config_result_schema_mmp_delta_limit() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
}
