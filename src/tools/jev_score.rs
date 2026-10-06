use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::enums::typesafe_state::TypesafeState;
use crate::enums::typesafe_question::Question;
use crate::clients::typesafe_client::TypesafeClient;
use crate::models::typesafe_response::TypesafeResponse;
use crate::enums::typesafe_error::TypesafeError;
use crate::utils::constants::SCORE_CRITERIA_MIN;
use crate::utils::constants::SCORE_CRITERIA_MAX;
use crate::utils::tool_trace::ToolInvocation;


#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct JevScoreParams {
    pub state: TypesafeState,
    pub instructions: TypesafeState,
    /// Ordered level descriptions. A Score needs at least 2 levels; the API accepts up to 10.
    #[schemars(length(min = SCORE_CRITERIA_MIN, max = SCORE_CRITERIA_MAX))]
    pub criteria: Vec<TypesafeState>,
}

pub async fn run_jev_score(
    params: JevScoreParams,
    client: &TypesafeClient,
    inv: &ToolInvocation,
) -> Result<TypesafeResponse, TypesafeError> {
    let n = params.criteria.len();
    if n < SCORE_CRITERIA_MIN || n > SCORE_CRITERIA_MAX {
        return Err(TypesafeError::Validation(format!(
            "criteria must have between {SCORE_CRITERIA_MIN} and {SCORE_CRITERIA_MAX} levels, got {n}"
        )));
    }

    let mut questions = HashMap::new();
    questions.insert("result".to_string(), Question::Score {
        instructions: Some(params.instructions),
        criteria: params.criteria,
    });
    let response = client.evaluate(inv, params.state, questions).await?;
    Ok(response)
}
