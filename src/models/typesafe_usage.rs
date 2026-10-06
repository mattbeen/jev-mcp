use serde::{Serialize, Deserialize};
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TypesafeUsage {
    pub input_tokens: u64,
    pub output_tokens: u64,
}