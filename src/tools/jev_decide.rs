use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;
use crate::enums::typesafe_state::TypesafeState;
use crate::enums::typesafe_question::Question;
use crate::clients::typesafe_client::TypesafeClient;
use crate::models::typesafe_response::TypesafeResponse;
use crate::enums::typesafe_error::TypesafeError;
use crate::utils::constants::DECIDE_QUESTIONS_MIN;
use crate::utils::constants::DECIDE_QUESTIONS_MAX;
use crate::utils::constants::CHOICE_CRITERIA_MIN;
use crate::utils::constants::CHOICE_CRITERIA_MAX;
use crate::utils::constants::SCORE_CRITERIA_MIN;
use crate::utils::constants::SCORE_CRITERIA_MAX;
use crate::utils::tool_trace::ToolInvocation;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct JevDecideParams {
    pub state: TypesafeState,
    #[schemars(length(min = DECIDE_QUESTIONS_MIN, max = DECIDE_QUESTIONS_MAX))]
    pub questions: HashMap<String, Question>,
}
pub async fn run_jev_decide(
    params: JevDecideParams,
    client: &TypesafeClient,
    inv: &ToolInvocation,
) -> Result<TypesafeResponse, TypesafeError> {
    let n = params.questions.len();
    if n < DECIDE_QUESTIONS_MIN || n > DECIDE_QUESTIONS_MAX {
        return Err(TypesafeError::Validation(format!("questions must have between {DECIDE_QUESTIONS_MIN} and {DECIDE_QUESTIONS_MAX} entries, got {n}")));
    }
    for (id,question) in &params.questions {
        match question {
            Question::Choice { criteria, instructions: _ } => {
                let count = criteria.len();
                if count < CHOICE_CRITERIA_MIN || count > CHOICE_CRITERIA_MAX {
                    return Err(TypesafeError::Validation(format!(
                        "question `{id}`: choice criteria must have between {CHOICE_CRITERIA_MIN} and {CHOICE_CRITERIA_MAX} options, got {count}"
                    )));
                }
            }
            Question::Score { criteria, instructions: _ } => {
                let count = criteria.len();
                if count < SCORE_CRITERIA_MIN || count > SCORE_CRITERIA_MAX {
                    return Err(TypesafeError::Validation(format!(
                        "question `{id}`: score criteria must have between {SCORE_CRITERIA_MIN} and {SCORE_CRITERIA_MAX} levels, got {count}"
                    )));
                }
            }
            Question::Noul { .. } => {}
        }
    }
    let response = client.evaluate(inv, params.state, params.questions).await?;
    Ok(response)
}