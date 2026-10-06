use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use schemars::JsonSchema;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "lowercase", deny_unknown_fields)]
pub enum Answer {
    Noul {
        noul: f64,
    },
    Choice {
        choice: String,
        probabilities: HashMap<String, f64>,
        confidence: f64,
    },
    Score {
        score: f64,
        legend: HashMap<String, String>,
        probabilities: HashMap<String, f64>,
        confidence: f64,
    },
}