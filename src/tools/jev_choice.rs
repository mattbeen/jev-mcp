use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::enums::typesafe_state::TypesafeState;
use crate::models::typesafe_response::TypesafeResponse;
use crate::enums::typesafe_error::TypesafeError;
use crate::enums::typesafe_question::Question;
use crate::clients::typesafe_client::TypesafeClient;
use crate::utils::constants::CHOICE_CRITERIA_MIN;
use crate::utils::constants::CHOICE_CRITERIA_MAX;
use crate::utils::tool_trace::ToolInvocation;


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct JevChoiceParams {
    pub state: TypesafeState,
    pub instructions: TypesafeState,
    /// Map of option id to rubric description; use null when an option needs no extra detail. Requires 2–255 options.
    #[schemars(length(min = CHOICE_CRITERIA_MIN, max = CHOICE_CRITERIA_MAX))]
    pub criteria: HashMap<String, Option<TypesafeState>>,
}

pub async fn run_jev_choice(
    params: JevChoiceParams,
    client: &TypesafeClient,
    inv: &ToolInvocation,
) -> Result<TypesafeResponse, TypesafeError> {
    let n = params.criteria.len();
    if n < CHOICE_CRITERIA_MIN || n > CHOICE_CRITERIA_MAX {
        return Err(TypesafeError::Validation(format!(
            "criteria must have between {CHOICE_CRITERIA_MIN} and {CHOICE_CRITERIA_MAX} options, got {n}"
        )));
    }

    let mut questions = HashMap::new();
    questions.insert(
        "result".to_string(),
        Question::Choice {
            instructions: Some(params.instructions),
            criteria: params.criteria,
        },
    );
    let response = client.evaluate(inv, params.state, questions).await?;
    Ok(response)
}
