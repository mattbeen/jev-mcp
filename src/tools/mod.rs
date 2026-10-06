use std::future::Future;
use serde::Serialize;
use rmcp::Json;
use crate::enums::typesafe_error::TypesafeError;
use crate::models::typesafe_response::TypesafeResponse;
use crate::utils::tool_trace::{
    ToolInvocation, log_mcp_tool_complete, log_mcp_tool_start,
};

pub mod jev_noul;
pub mod jev_score;
pub mod jev_choice;
pub mod jev_decide;

pub async fn run_tool<P, F, Fut>(
    tool_name: &'static str,
    params: P,
    f: F,
) -> Result<Json<TypesafeResponse>, String>
where
    P: Serialize,
    F: FnOnce(ToolInvocation, P) -> Fut,
    Fut: Future<Output = Result<TypesafeResponse, TypesafeError>>,
{
    let inv = ToolInvocation::new(tool_name);
    log_mcp_tool_start(&inv, &params);
    let result = f(inv.clone(), params).await;
    match &result {
        Ok(response) => log_mcp_tool_complete(&inv, Ok(response)),
        Err(error) => log_mcp_tool_complete(&inv, Err(error)),
    }
    result.map(Json).map_err(|error| error.to_string())
}
