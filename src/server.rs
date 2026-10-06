use std::sync::Arc;
use rmcp::{tool, tool_router,handler::server::wrapper::Parameters, Json};
use crate::clients::typesafe_client::TypesafeClient;
use crate::tools::jev_noul::{JevNoulParams, run_jev_noul};
use crate::tools::jev_score::{JevScoreParams, run_jev_score};
use crate::tools::jev_choice::{JevChoiceParams, run_jev_choice};
use crate::tools::jev_decide::{JevDecideParams, run_jev_decide};
use crate::tools::run_tool;
use crate::models::typesafe_response::TypesafeResponse;


#[derive(Clone)]
pub struct JevServer {
    pub client: Arc<TypesafeClient>,
}

#[tool_router(server_handler)]
impl JevServer {
    #[tool(
        name = "jev_noul",
        title = "Evaluate a binary proposition",
        description = "Use Jev Noul to estimate whether a proposition is true from the supplied state and instructions. Optionally provide criteria with true and/or false definitions. Validation: state and instructions are required and must be text, an object, or an array; criteria may contain only true and false. Returns the calibrated probability in answers.result.noul.",
    )]
    pub async fn jev_noul(&self, Parameters(params): Parameters<JevNoulParams>) -> Result<Json<TypesafeResponse>, String> {
        run_tool("jev_noul", params, |inv, params| async move {
            run_jev_noul(params, &self.client, &inv).await
        }).await
    }

    #[tool(
        name = "jev_score",
        title = "Score on an ordered scale",
        description = "Use Jev Score to evaluate state against an ordered rubric. Provide state, instructions, and criteria in the intended scale order. Validation: criteria is required and must contain between 2 and 10 levels; each level must be text, an object, or an array. Returns the score, legend, probability distribution, and confidence in answers.result.",
    )]
    pub async fn jev_score(&self, Parameters(params): Parameters<JevScoreParams>) -> Result<Json<TypesafeResponse>, String> {
        run_tool("jev_score", params, |inv, params| async move {
            run_jev_score(params, &self.client, &inv).await
        }).await
    }

    #[tool(
        name = "jev_choice",
        title = "Choose among defined options",
        description = "Use Jev Choice to select one option from a defined set. Provide state, instructions, and criteria mapping each option identifier to an optional description; use null when no description is needed. Validation: criteria is required and must contain between 2 and 255 options; each description must be null, text, an object, or an array. Returns the selected option, probability distribution, and confidence in answers.result.",
    )]
    pub async fn jev_choice(&self, Parameters(params): Parameters<JevChoiceParams>) -> Result<Json<TypesafeResponse>, String> {
        run_tool("jev_choice", params, |inv, params| async move {
            run_jev_choice(params, &self.client, &inv).await
        }).await
    }

    #[tool(
        name = "jev_decide",
        title = "Batch mixed Jev judgments",
        description = "Evaluate 1–32 independent Noul, Choice, and/or Score questions over one shared state in a single TypeSafe request. Each questions map key is the answer id. Questions cannot see each other's answers; state premises in instructions if a question is speculative. Choice options must be 2–255; Score levels must be 2–10. Returns answers keyed by those ids plus usage. Model is server-configured.",
    )]
    pub async fn jev_decide(&self, Parameters(params): Parameters<JevDecideParams>) -> Result<Json<TypesafeResponse>, String> {
        run_tool("jev_decide", params, |inv, params| async move {
            run_jev_decide(params, &self.client, &inv).await
        }).await
    }
}
