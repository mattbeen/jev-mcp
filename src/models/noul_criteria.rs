use serde::{Serialize, Deserialize};
use crate::enums::typesafe_state::TypesafeState;
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct NoulCriteria {
    #[serde(default, rename = "true", skip_serializing_if = "Option::is_none")]
    pub true_criteria: Option<TypesafeState>,
    #[serde(default, rename = "false", skip_serializing_if = "Option::is_none")]
    pub false_criteria: Option<TypesafeState>,
}