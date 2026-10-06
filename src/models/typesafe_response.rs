use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::enums::typesafe_answer::Answer;
use crate::models::typesafe_usage::TypesafeUsage;
use schemars::JsonSchema;


#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypesafeResponse {
    pub model: String,
    pub answers: HashMap<String, Answer>,
    pub usage: TypesafeUsage,
}