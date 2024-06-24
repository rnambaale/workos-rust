use crate::errors::WorkOsError;

pub struct WorkOs {
    pub key: String,
    pub base_url: String,
    pub api_key: Option<String>,
}

#[allow(dead_code)]
impl WorkOs {
    pub fn new(new_key: &str) -> Self {
        Self {
            key: new_key.to_string(),
            base_url: "https://api.workos.com/".to_string(),
            api_key: None,
        }
    }

    pub fn new_with_url(new_key: &str, base_url: &str) -> Self {
        Self {
            key: new_key.to_string(),
            base_url: base_url.to_string(),
            api_key: None,
        }
    }

    pub fn get_api_key(&self) -> Result<String, WorkOsError> {
        if let Some(api_key) = &self.api_key {
            return Ok(api_key.clone());
        }

        // dotenv().ok();

        // if let Ok(api_key) = std::env::var("WORKOS_API_KEY") {
        //     return Ok(api_key);
        // }

        Err(WorkOsError::ConfigurationError("WORKOS_API_KEY should be set".to_string()))
    }

    pub fn set_api_key(&mut self, key: &str) {
        self.api_key = Some(key.to_string());
    }
}
