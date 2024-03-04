#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Cancel all open orders for a given subaccount and a given instrument.
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/cancel_by_instrument",
  "description": "Cancel all open orders for a given subaccount and a given instrument.",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateCancelByInstrumentJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateCancelByInstrument(pub PrivateCancelByInstrumentJsonrpcSchema);
impl std::ops::Deref for PrivateCancelByInstrument {
    type Target = PrivateCancelByInstrumentJsonrpcSchema;
    fn deref(&self) -> &PrivateCancelByInstrumentJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateCancelByInstrument> for PrivateCancelByInstrumentJsonrpcSchema {
    fn from(value: PrivateCancelByInstrument) -> Self {
        value.0
    }
}
impl From<&PrivateCancelByInstrument> for PrivateCancelByInstrument {
    fn from(value: &PrivateCancelByInstrument) -> Self {
        value.clone()
    }
}
impl From<PrivateCancelByInstrumentJsonrpcSchema> for PrivateCancelByInstrument {
    fn from(value: PrivateCancelByInstrumentJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateCancelByInstrumentJsonrpcSchema
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
      "$ref": "#/definitions/PrivateCancelByInstrumentRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelByInstrumentResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByInstrumentJsonrpcSchema {
    pub request: PrivateCancelByInstrumentRequestSchema,
    pub response: PrivateCancelByInstrumentResponseSchema,
}
impl From<&PrivateCancelByInstrumentJsonrpcSchema>
for PrivateCancelByInstrumentJsonrpcSchema {
    fn from(value: &PrivateCancelByInstrumentJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByInstrumentParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "instrument_name",
    "subaccount_id"
  ],
  "properties": {
    "instrument_name": {
      "title": "instrument_name",
      "description": "Cancel all orders for this instrument",
      "type": "string"
    },
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

pub struct PrivateCancelByInstrumentParamsSchema {
    ///Cancel all orders for this instrument
    pub instrument_name: String,
    ///Subaccount ID
    pub subaccount_id: i64,
}
impl From<&PrivateCancelByInstrumentParamsSchema>
for PrivateCancelByInstrumentParamsSchema {
    fn from(value: &PrivateCancelByInstrumentParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByInstrumentRequestSchema
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
      "const": "private/cancel_by_instrument"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateCancelByInstrumentParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByInstrumentRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateCancelByInstrumentRequestSchemaId>,
    pub method: String,
    pub params: PrivateCancelByInstrumentParamsSchema,
}
impl From<&PrivateCancelByInstrumentRequestSchema>
for PrivateCancelByInstrumentRequestSchema {
    fn from(value: &PrivateCancelByInstrumentRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByInstrumentRequestSchemaId
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
pub enum PrivateCancelByInstrumentRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelByInstrumentRequestSchemaId>
for PrivateCancelByInstrumentRequestSchemaId {
    fn from(value: &PrivateCancelByInstrumentRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelByInstrumentRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelByInstrumentRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelByInstrumentRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelByInstrumentRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelByInstrumentRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelByInstrumentRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelByInstrumentResponseSchema
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
      "$ref": "#/definitions/PrivateCancelByInstrumentResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByInstrumentResponseSchema {
    pub id: PrivateCancelByInstrumentResponseSchemaId,
    ///
    pub result: PrivateCancelByInstrumentResultSchema,
}
impl From<&PrivateCancelByInstrumentResponseSchema>
for PrivateCancelByInstrumentResponseSchema {
    fn from(value: &PrivateCancelByInstrumentResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateCancelByInstrumentResponseSchemaId
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
pub enum PrivateCancelByInstrumentResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateCancelByInstrumentResponseSchemaId>
for PrivateCancelByInstrumentResponseSchemaId {
    fn from(value: &PrivateCancelByInstrumentResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateCancelByInstrumentResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateCancelByInstrumentResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateCancelByInstrumentResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateCancelByInstrumentResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateCancelByInstrumentResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateCancelByInstrumentResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateCancelByInstrumentResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "cancelled_orders"
  ],
  "properties": {
    "cancelled_orders": {
      "title": "cancelled_orders",
      "description": "Number of cancelled orders",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateCancelByInstrumentResultSchema {
    ///Number of cancelled orders
    pub cancelled_orders: i64,
}
impl From<&PrivateCancelByInstrumentResultSchema>
for PrivateCancelByInstrumentResultSchema {
    fn from(value: &PrivateCancelByInstrumentResultSchema) -> Self {
        value.clone()
    }
}
