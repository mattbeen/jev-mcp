use std::time::Duration;
use reqwest::Client;
use crate::config::settings::Settings;
use std::sync::Arc;

pub struct HttpClientManager{
    client: Arc<Client>,
}
impl HttpClientManager{
    pub fn new(settings: &Settings) -> Result<Self, reqwest::Error> {
        let client = Client::builder()
            .timeout(Duration::from_secs(settings.request_timeout))
            .build()?;
        Ok(HttpClientManager { client: Arc::new(client) })
    }
    pub fn client(&self) -> Arc<Client> {
        Arc::clone(&self.client)
    }
}