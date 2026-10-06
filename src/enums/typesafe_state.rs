use serde::{Serialize, Deserialize};
use serde_json::Value;
use std::collections::HashMap;
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum TypesafeState {
    Text(String),
    Object(HashMap<String, Value>),
    List(Vec<Value>),
}