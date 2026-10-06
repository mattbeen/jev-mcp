use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::models::noul_criteria::NoulCriteria;
use crate::enums::typesafe_state::TypesafeState;
use crate::enums::typesafe_question::Question;
use crate::clients::typesafe_client::TypesafeClient;
use crate::models::typesafe_response::TypesafeResponse;
use crate::enums::typesafe_error::TypesafeError;
use crate::utils::tool_trace::ToolInvocation;

#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct JevNoulParams {
    pub state: TypesafeState,
    pub instructions: TypesafeState,
    #[serde(default)]
    pub criteria: Option<NoulCriteria>,
}

pub async fn run_jev_noul(
    params: JevNoulParams,
    client: &TypesafeClient,
    inv: &ToolInvocation,
) -> Result<TypesafeResponse, TypesafeError> {
    let mut questions = HashMap::new();
    questions.insert("result".to_string(), Question::Noul {
        instructions: Some(params.instructions),
        criteria: params.criteria,
    });
    let response = client.evaluate(inv, params.state, questions).await?;
    Ok(response)
}