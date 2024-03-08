#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Resets (unfreezes) the mmp state for a subaccount (optionally filtered by currency)
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/reset_mmp",
  "description": "Resets (unfreezes) the mmp state for a subaccount (optionally filtered by currency)",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateResetMmpJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateResetMmp(pub PrivateResetMmpJsonrpcSchema);
impl std::ops::Deref for PrivateResetMmp {
    type Target = PrivateResetMmpJsonrpcSchema;
    fn deref(&self) -> &PrivateResetMmpJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateResetMmp> for PrivateResetMmpJsonrpcSchema {
    fn from(value: PrivateResetMmp) -> Self {
        value.0
    }
}
impl From<&PrivateResetMmp> for PrivateResetMmp {
    fn from(value: &PrivateResetMmp) -> Self {
        value.clone()
    }
}
impl From<PrivateResetMmpJsonrpcSchema> for PrivateResetMmp {
    fn from(value: PrivateResetMmpJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateResetMmpJsonrpcSchema
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
      "$ref": "#/definitions/PrivateResetMmpRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateResetMmpResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateResetMmpJsonrpcSchema {
    pub request: PrivateResetMmpRequestSchema,
    pub response: PrivateResetMmpResponseSchema,
}
impl From<&PrivateResetMmpJsonrpcSchema> for PrivateResetMmpJsonrpcSchema {
    fn from(value: &PrivateResetMmpJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateResetMmpParamsSchema
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
      "description": "Currency to reset the mmp for. If not provided, resets all configs for the subaccount",
      "type": [
        "string",
        "null"
      ]
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id for which to reset the mmp state",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateResetMmpParamsSchema {
    ///Currency to reset the mmp for. If not provided, resets all configs for the subaccount
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Subaccount_id for which to reset the mmp state
    pub subaccount_id: i64,
}
impl From<&PrivateResetMmpParamsSchema> for PrivateResetMmpParamsSchema {
    fn from(value: &PrivateResetMmpParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateResetMmpRequestSchema
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
      "const": "private/reset_mmp"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateResetMmpParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateResetMmpRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateResetMmpRequestSchemaId>,
    pub method: String,
    pub params: PrivateResetMmpParamsSchema,
}
impl From<&PrivateResetMmpRequestSchema> for PrivateResetMmpRequestSchema {
    fn from(value: &PrivateResetMmpRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateResetMmpRequestSchemaId
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
pub enum PrivateResetMmpRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateResetMmpRequestSchemaId> for PrivateResetMmpRequestSchemaId {
    fn from(value: &PrivateResetMmpRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateResetMmpRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateResetMmpRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateResetMmpRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateResetMmpRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateResetMmpRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateResetMmpRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateResetMmpResponseSchema
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
      "description": "The result of this method call, `ok` if successful",
      "type": "string",
      "const": "ok"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateResetMmpResponseSchema {
    pub id: PrivateResetMmpResponseSchemaId,
    ///The result of this method call, `ok` if successful
    pub result: String,
}
impl From<&PrivateResetMmpResponseSchema> for PrivateResetMmpResponseSchema {
    fn from(value: &PrivateResetMmpResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateResetMmpResponseSchemaId
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
pub enum PrivateResetMmpResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateResetMmpResponseSchemaId> for PrivateResetMmpResponseSchemaId {
    fn from(value: &PrivateResetMmpResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateResetMmpResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateResetMmpResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateResetMmpResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateResetMmpResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateResetMmpResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateResetMmpResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
