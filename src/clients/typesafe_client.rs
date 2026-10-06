use std::sync::Arc;
use std::collections::HashMap;
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use crate::utils::http_client::HttpClientManager;
use crate::config::settings::Settings;
use crate::enums::typesafe_error::TypesafeError;
use crate::enums::typesafe_state::TypesafeState;
use crate::enums::typesafe_question::Question;
use crate::models::typesafe_response::TypesafeResponse;
use crate::models::typesafe_request::TypesafeRequest;
use crate::utils::tool_trace::{
    question_types, ToolInvocation, log_jev_error, log_jev_request, log_jev_response,
};

pub struct TypesafeClient {
    http_client: Arc<Client>,
    api_url: String,
    api_key: String,
    model: String,
}
impl TypesafeClient {
    pub fn new(http_client: &HttpClientManager,settings: &Settings) -> Self {
        Self {
            http_client: http_client.client(),
            api_url: settings.typesafe_api_url.clone(),
            api_key: settings.typesafe_api_key.clone(),
            model: settings.model.clone(),
        }
    }

    pub async fn evaluate(
        &self,
        inv: &ToolInvocation,
        state: TypesafeState,
        questions: HashMap<String, Question>,
    ) -> Result<TypesafeResponse, TypesafeError> {
        let jev_types = question_types(inv.tool_name, &questions);
        let request = TypesafeRequest {
            state,
            model: self.model.clone(),
            questions,
        };
        log_jev_request(inv, &jev_types, &request);

        let result = async {
            let response = self
                .http_client
                .post(&self.api_url)
                .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
                .header(CONTENT_TYPE, "application/json")
                .json(&request)
                .send()
                .await?;

            let status = response.status();
            if !status.is_success() {
                let body = response.text().await.unwrap_or_default();
                return Err(TypesafeError::Api { status, body });
            }
            let parsed_response = response.json::<TypesafeResponse>().await?;
            Ok(parsed_response)
        }
        .await;

        match &result {
            Ok(response) => log_jev_response(inv, response),
            Err(error) => log_jev_error(inv, error),
        }
        result
    }
}