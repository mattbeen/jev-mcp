use serde::{Serialize, Deserialize};
use crate::enums::typesafe_state::TypesafeState;
use crate::models::noul_criteria::NoulCriteria;
use std::collections::HashMap;
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize,JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Question {
    Noul {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        instructions: Option<TypesafeState>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        criteria: Option<NoulCriteria>,
    },
    Choice {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        instructions: Option<TypesafeState>,
        criteria: HashMap<String, Option<TypesafeState>>,
    },
    Score {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        instructions: Option<TypesafeState>,
        criteria: Vec<TypesafeState>,
    },
}