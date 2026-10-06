use serde::Serialize;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Instant;
use uuid::Uuid;

use crate::enums::typesafe_error::TypesafeError;
use crate::enums::typesafe_question::Question;
use crate::models::typesafe_request::TypesafeRequest;
use crate::models::typesafe_response::TypesafeResponse;

#[derive(Clone)]
pub struct ToolInvocation {
    pub id: String,
    pub tool_name: &'static str,
    started: Instant,
}

impl ToolInvocation {
    pub fn new(tool_name: &'static str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            tool_name,
            started: Instant::now(),
        }
    }

    pub fn duration_ms(&self) -> u64 {
        self.started.elapsed().as_millis() as u64
    }
}

fn to_log_value<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap_or_else(|_| json!("<serialize_error>"))
}

fn question_type_name(question: &Question) -> &'static str {
    match question {
        Question::Noul { .. } => "noul",
        Question::Choice { .. } => "choice",
        Question::Score { .. } => "score",
    }
}

pub fn question_types(tool_name: &str, questions: &HashMap<String, Question>) -> Value {
    if tool_name == "jev_decide" {
        let map: HashMap<&String, &str> = questions
            .iter()
            .map(|(id, question)| (id, question_type_name(question)))
            .collect();
        json!(map)
    } else {
        questions
            .values()
            .next()
            .map(|question| json!(question_type_name(question)))
            .unwrap_or(Value::Null)
    }
}

fn emit(event: &str, fields: Value) {
    let invocation_id = fields
        .get("invocation_id")
        .and_then(|value| value.as_str())
        .unwrap_or("");
    tracing::info!(
        target: "jev_mcp",
        event,
        invocation_id,
        payload = %fields,
    );
}

pub fn log_mcp_tool_start(inv: &ToolInvocation, params: &impl Serialize) {
    emit(
        "mcp_tool_start",
        json!({
            "invocation_id": inv.id,
            "tool_name": inv.tool_name,
            "mcp_input": to_log_value(params),
        }),
    );
}

pub fn log_jev_request(inv: &ToolInvocation, jev_types: &Value, request: &TypesafeRequest) {
    emit(
        "jev_request",
        json!({
            "invocation_id": inv.id,
            "tool_name": inv.tool_name,
            "jev_types": jev_types,
            "jev_request": to_log_value(request),
        }),
    );
}

pub fn log_jev_response(inv: &ToolInvocation, response: &TypesafeResponse) {
    emit(
        "jev_response",
        json!({
            "invocation_id": inv.id,
            "jev_response": to_log_value(response),
        }),
    );
}

pub fn log_jev_error(inv: &ToolInvocation, error: &TypesafeError) {
    emit(
        "jev_error",
        json!({
            "invocation_id": inv.id,
            "error": error.to_string(),
        }),
    );
}

pub fn log_mcp_tool_complete(
    inv: &ToolInvocation,
    result: Result<&TypesafeResponse, &TypesafeError>,
) {
    let tool_result = match result {
        Ok(response) => json!({ "ok": to_log_value(response) }),
        Err(error) => json!({ "err": error.to_string() }),
    };
    emit(
        "mcp_tool_complete",
        json!({
            "invocation_id": inv.id,
            "tool_name": inv.tool_name,
            "tool_result": tool_result,
            "duration_ms": inv.duration_ms(),
        }),
    );
}
