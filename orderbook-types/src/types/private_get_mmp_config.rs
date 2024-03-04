#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///MmpConfigParamsSchema
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

pub struct MmpConfigParamsSchema {
    ///Currency of this mmp config
    pub currency: String,
    ///Maximum total order amount that can be traded within the mmp_interval across all instruments of the provided currency. The amounts are not netted, so a filled bid of 1 and a filled ask of 2 would count as 3.<br />Default: 0 (no limit)
    #[serde(default = "defaults::mmp_config_params_schema_mmp_amount_limit")]
    pub mmp_amount_limit: bigdecimal::BigDecimal,
    ///Maximum total delta that can be traded within the mmp_interval across all instruments of the provided currency. This quantity is netted, so a filled order with +1 delta and a filled order with -2 delta would count as -1<br />Default: 0 (no limit)
    #[serde(default = "defaults::mmp_config_params_schema_mmp_delta_limit")]
    pub mmp_delta_limit: bigdecimal::BigDecimal,
    ///Time interval in ms setting how long the subaccount is frozen after an mmp trigger, if 0 then a manual reset would be required via private/reset_mmp
    pub mmp_frozen_time: i64,
    ///Time interval in ms over which the limits are monotored, if 0 then mmp is disabled
    pub mmp_interval: i64,
    ///Subaccount_id for which to set the config
    pub subaccount_id: i64,
}
impl From<&MmpConfigParamsSchema> for MmpConfigParamsSchema {
    fn from(value: &MmpConfigParamsSchema) -> Self {
        value.clone()
    }
}
///Get the current mmp config for a subaccount (optionally filtered by currency)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/get_mmp_config",
  "description": "Get the current mmp config for a subaccount (optionally filtered by currency)",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateGetMmpConfigJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateGetMmpConfig(pub PrivateGetMmpConfigJsonrpcSchema);
impl std::ops::Deref for PrivateGetMmpConfig {
    type Target = PrivateGetMmpConfigJsonrpcSchema;
    fn deref(&self) -> &PrivateGetMmpConfigJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateGetMmpConfig> for PrivateGetMmpConfigJsonrpcSchema {
    fn from(value: PrivateGetMmpConfig) -> Self {
        value.0
    }
}
impl From<&PrivateGetMmpConfig> for PrivateGetMmpConfig {
    fn from(value: &PrivateGetMmpConfig) -> Self {
        value.clone()
    }
}
impl From<PrivateGetMmpConfigJsonrpcSchema> for PrivateGetMmpConfig {
    fn from(value: PrivateGetMmpConfigJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateGetMmpConfigJsonrpcSchema
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
      "$ref": "#/definitions/PrivateGetMmpConfigRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetMmpConfigResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetMmpConfigJsonrpcSchema {
    pub request: PrivateGetMmpConfigRequestSchema,
    pub response: PrivateGetMmpConfigResponseSchema,
}
impl From<&PrivateGetMmpConfigJsonrpcSchema> for PrivateGetMmpConfigJsonrpcSchema {
    fn from(value: &PrivateGetMmpConfigJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMmpConfigParamsSchema
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
    "currency": {
      "title": "currency",
      "description": "Currency to get the config for. If not provided, returns all configs for the subaccount",
      "default": null,
      "type": [
        "string",
        "null"
      ]
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to get the config",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetMmpConfigParamsSchema {
    ///Currency to get the config for. If not provided, returns all configs for the subaccount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Subaccount_id for which to get the config
    pub subaccount_id: i64,
}
impl From<&PrivateGetMmpConfigParamsSchema> for PrivateGetMmpConfigParamsSchema {
    fn from(value: &PrivateGetMmpConfigParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMmpConfigRequestSchema
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
      "const": "private/get_mmp_config"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateGetMmpConfigParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetMmpConfigRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateGetMmpConfigRequestSchemaId>,
    pub method: String,
    pub params: PrivateGetMmpConfigParamsSchema,
}
impl From<&PrivateGetMmpConfigRequestSchema> for PrivateGetMmpConfigRequestSchema {
    fn from(value: &PrivateGetMmpConfigRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMmpConfigRequestSchemaId
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
pub enum PrivateGetMmpConfigRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetMmpConfigRequestSchemaId> for PrivateGetMmpConfigRequestSchemaId {
    fn from(value: &PrivateGetMmpConfigRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetMmpConfigRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetMmpConfigRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetMmpConfigRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetMmpConfigRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetMmpConfigRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetMmpConfigRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateGetMmpConfigResponseSchema
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
      "title": "result",
      "description": "",
      "type": "array",
      "items": {
        "type": "object",
        "$ref": "#/definitions/MMPConfigParamsSchema",
        "field_many": true
      }
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateGetMmpConfigResponseSchema {
    pub id: PrivateGetMmpConfigResponseSchemaId,
    ///
    pub result: Vec<MmpConfigParamsSchema>,
}
impl From<&PrivateGetMmpConfigResponseSchema> for PrivateGetMmpConfigResponseSchema {
    fn from(value: &PrivateGetMmpConfigResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateGetMmpConfigResponseSchemaId
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
pub enum PrivateGetMmpConfigResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateGetMmpConfigResponseSchemaId> for PrivateGetMmpConfigResponseSchemaId {
    fn from(value: &PrivateGetMmpConfigResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateGetMmpConfigResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateGetMmpConfigResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateGetMmpConfigResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateGetMmpConfigResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateGetMmpConfigResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateGetMmpConfigResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
pub mod defaults {
    pub(super) fn mmp_config_params_schema_mmp_amount_limit() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
    pub(super) fn mmp_config_params_schema_mmp_delta_limit() -> bigdecimal::BigDecimal {
        serde_json::from_str::<bigdecimal::BigDecimal>("\"0\"").unwrap()
    }
}
