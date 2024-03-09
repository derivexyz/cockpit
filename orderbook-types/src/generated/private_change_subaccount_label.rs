#![allow(unused_variables)]
#![allow(unused_imports)]
use serde::{Deserialize, Serialize};
use bigdecimal;
use uuid;
///Change a user defined label for given subaccount
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "title": "private/change_subaccount_label",
  "description": "Change a user defined label for given subaccount",
  "type": "object",
  "allOf": [
    {
      "$ref": "#/definitions/PrivateChangeSubaccountLabelJSONRPCSchema"
    }
  ]
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PrivateChangeSubaccountLabel(pub PrivateChangeSubaccountLabelJsonrpcSchema);
impl std::ops::Deref for PrivateChangeSubaccountLabel {
    type Target = PrivateChangeSubaccountLabelJsonrpcSchema;
    fn deref(&self) -> &PrivateChangeSubaccountLabelJsonrpcSchema {
        &self.0
    }
}
impl From<PrivateChangeSubaccountLabel> for PrivateChangeSubaccountLabelJsonrpcSchema {
    fn from(value: PrivateChangeSubaccountLabel) -> Self {
        value.0
    }
}
impl From<&PrivateChangeSubaccountLabel> for PrivateChangeSubaccountLabel {
    fn from(value: &PrivateChangeSubaccountLabel) -> Self {
        value.clone()
    }
}
impl From<PrivateChangeSubaccountLabelJsonrpcSchema> for PrivateChangeSubaccountLabel {
    fn from(value: PrivateChangeSubaccountLabelJsonrpcSchema) -> Self {
        Self(value)
    }
}
///PrivateChangeSubaccountLabelJsonrpcSchema
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
      "$ref": "#/definitions/PrivateChangeSubaccountLabelRequestSchema",
      "field_many": false
    },
    "response": {
      "type": "object",
      "$ref": "#/definitions/PrivateChangeSubaccountLabelResponseSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSubaccountLabelJsonrpcSchema {
    pub request: PrivateChangeSubaccountLabelRequestSchema,
    pub response: PrivateChangeSubaccountLabelResponseSchema,
}
impl From<&PrivateChangeSubaccountLabelJsonrpcSchema>
for PrivateChangeSubaccountLabelJsonrpcSchema {
    fn from(value: &PrivateChangeSubaccountLabelJsonrpcSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSubaccountLabelParamsSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "label",
    "subaccount_id"
  ],
  "properties": {
    "label": {
      "title": "label",
      "description": "User defined label",
      "type": "string"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSubaccountLabelParamsSchema {
    ///User defined label
    pub label: String,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateChangeSubaccountLabelParamsSchema>
for PrivateChangeSubaccountLabelParamsSchema {
    fn from(value: &PrivateChangeSubaccountLabelParamsSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSubaccountLabelRequestSchema
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
      "const": "private/change_subaccount_label"
    },
    "params": {
      "type": "object",
      "$ref": "#/definitions/PrivateChangeSubaccountLabelParamsSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSubaccountLabelRequestSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<PrivateChangeSubaccountLabelRequestSchemaId>,
    pub method: String,
    pub params: PrivateChangeSubaccountLabelParamsSchema,
}
impl From<&PrivateChangeSubaccountLabelRequestSchema>
for PrivateChangeSubaccountLabelRequestSchema {
    fn from(value: &PrivateChangeSubaccountLabelRequestSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSubaccountLabelRequestSchemaId
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
pub enum PrivateChangeSubaccountLabelRequestSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateChangeSubaccountLabelRequestSchemaId>
for PrivateChangeSubaccountLabelRequestSchemaId {
    fn from(value: &PrivateChangeSubaccountLabelRequestSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateChangeSubaccountLabelRequestSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateChangeSubaccountLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateChangeSubaccountLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateChangeSubaccountLabelRequestSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateChangeSubaccountLabelRequestSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateChangeSubaccountLabelRequestSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateChangeSubaccountLabelResponseSchema
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
      "$ref": "#/definitions/PrivateChangeSubaccountLabelResultSchema",
      "field_many": false
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSubaccountLabelResponseSchema {
    pub id: PrivateChangeSubaccountLabelResponseSchemaId,
    ///
    pub result: PrivateChangeSubaccountLabelResultSchema,
}
impl From<&PrivateChangeSubaccountLabelResponseSchema>
for PrivateChangeSubaccountLabelResponseSchema {
    fn from(value: &PrivateChangeSubaccountLabelResponseSchema) -> Self {
        value.clone()
    }
}
///PrivateChangeSubaccountLabelResponseSchemaId
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
pub enum PrivateChangeSubaccountLabelResponseSchemaId {
    Variant0(String),
    Variant1(i64),
}
impl From<&PrivateChangeSubaccountLabelResponseSchemaId>
for PrivateChangeSubaccountLabelResponseSchemaId {
    fn from(value: &PrivateChangeSubaccountLabelResponseSchemaId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for PrivateChangeSubaccountLabelResponseSchemaId {
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
impl std::convert::TryFrom<&str> for PrivateChangeSubaccountLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for PrivateChangeSubaccountLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for PrivateChangeSubaccountLabelResponseSchemaId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for PrivateChangeSubaccountLabelResponseSchemaId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
            Self::Variant1(x) => x.to_string(),
        }
    }
}
impl From<i64> for PrivateChangeSubaccountLabelResponseSchemaId {
    fn from(value: i64) -> Self {
        Self::Variant1(value)
    }
}
///PrivateChangeSubaccountLabelResultSchema
///
/// <details><summary>JSON schema</summary>
///
/// ```json
/**{
  "type": "object",
  "required": [
    "label",
    "subaccount_id"
  ],
  "properties": {
    "label": {
      "title": "label",
      "description": "User defined label",
      "type": "string"
    },
    "subaccount_id": {
      "title": "subaccount_id",
      "description": "Subaccount_id",
      "type": "integer"
    }
  },
  "additionalProperties": false
}*/
/// ```
/// </details>
#[derive(Clone, Debug, Deserialize, Serialize)]

pub struct PrivateChangeSubaccountLabelResultSchema {
    ///User defined label
    pub label: String,
    ///Subaccount_id
    pub subaccount_id: i64,
}
impl From<&PrivateChangeSubaccountLabelResultSchema>
for PrivateChangeSubaccountLabelResultSchema {
    fn from(value: &PrivateChangeSubaccountLabelResultSchema) -> Self {
        value.clone()
    }
}
