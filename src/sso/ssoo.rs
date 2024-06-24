use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct PostData {
    title: String,
    body: String,
    userId: u32,
}

#[derive(Deserialize, Debug, PartialEq)]
struct PostResponse {
    id: u32,
    title: String,
    body: String,
    userId: u32,
}

struct HttpClient {
    base_url: String,
    client: Client,
}

impl HttpClient {
    pub fn new(base_url: &str) -> Self {
        HttpClient {
            base_url: base_url.to_string(),
            client: Client::new(),
        }
    }

    pub fn post<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        endpoint: &str,
        data: &T,
    ) -> Result<R, Box<dyn std::error::Error>> {
        let url = format!("{}/{}", self.base_url, endpoint);
        let response = self.client.post(&url).json(data).send()?;

        if response.status().is_success() {
            let body = response.json::<R>()?;
            Ok(body)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito;

    #[test]
    fn test_post() {
        let mut server = mockito::Server::new();
        let mock = server.mock("POST", "/posts")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 101, "title": "foo", "body": "bar", "userId": 1}"#)
            .create();

        let _m = mock("POST", "/posts")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{"id": 101, "title": "foo", "body": "bar", "userId": 1}"#)
            .create();

        let client = HttpClient::new(&mockito::server_url());

        // let post_data = PostData {
        //     title: "foo".to_string(),
        //     body: "bar".to_string(),
        //     userId: 1,
        // };

        // let response: PostResponse = client.post("posts", &post_data).unwrap();

        // assert_eq!(
        //     response,
        //     PostResponse {
        //         id: 101,
        //         title: "foo".to_string(),
        //         body: "bar".to_string(),
        //         userId: 1
        //     }
        // );
    }
}
