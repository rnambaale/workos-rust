// use std::fmt::format;

use reqwest;
use serde::{Deserialize, Serialize};

use crate::workos::workos::WorkOs;

pub struct Sso {
    workos: WorkOs,
}

#[allow(dead_code)]
impl Sso {
    pub fn new(workos: WorkOs) -> Self {
        Self {
            workos,
        }
    }

    pub fn get_profile_and_token(&self, code: &str, client_id: &str) -> Result<ProfileAndToken, Box<dyn std::error::Error>> {
        let url = format!("{}/sso/token", self.workos.base_url);
        let data = PostData {
            client_id: client_id.to_string(),
            client_secret: self.workos.key.to_string(),
            code: code.to_string(),
            grant_type: "authorization_code".to_string(),
        };

        let client = reqwest::blocking::Client::new();
        let response = client.post(url).json(&data).send()?;

        if response.status().is_success() {
            let body = response.json::<ProfileAndToken>()?;
            Ok(body)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }

    pub fn get_profile(&self, access_token: &str) -> Result<Profile, Box<dyn std::error::Error>> {
        let url = format!("{}/sso/profile", self.workos.base_url);

        let client = reqwest::blocking::Client::new();
        let response = client.get(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .send()?;

        if response.status().is_success() {
            let body = response.json::<Profile>()?;
            Ok(body)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }

    pub fn get_connection(&self, connection_id: &str) -> Result<Connection, Box<dyn std::error::Error>>{
        let url = format!("{}/connections/{}", self.workos.base_url, connection_id);

        let client = reqwest::blocking::Client::new();
        let access_token: String = self.workos.get_api_key().unwrap();
        let response = client.get(url)
            .header("Authorization", format!("Bearer {access_token}"))
            .send()?;

        if response.status().is_success() {
            let body = response.json::<Connection>()?;
            Ok(body)
        } else {
            Err(format!("Request failed with status: {}", response.status()).into())
        }
    }
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ProfileAndToken {
    access_token: String,
    profile: Profile,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Profile {
    id: String,
    idp_id: String,
    // organization_id: Option<String>,
    connection_id: String,
    connection_type: ConnectionType,
    email: String,
    // first_name: Option<String>,
    // last_name: Option<String>,
    // groups: Option<Vec<String>>,
    // raw_attributes?: { [key: string]: any };
}

#[derive(Debug, Deserialize)]
enum ConnectionType {
    ADFSSAML,
    AdpOidc,
    Auth0SAML,
    AzureSAML,
    CasSAML,
    ClassLinkSAML,
    CloudflareSAML,
    CyberArkSAML,
    DuoSAML,
    GenericOIDC,
    GenericSAML,
    GoogleOAuth,
    GoogleSAML,
    JumpCloudSAML,
    KeycloakSAML,
    LastPassSAML,
    LoginGovOidc,
    MagicLink,
    MicrosoftOAuth,
    MiniOrangeSAML,
    NetIqSAML,
    OktaSAML,
    OneLoginSAML,
    OracleSAML,
    PingFederateSAML,
    PingOneSAML,
    RipplingSAML,
    SalesforceSAML,
    ShibbolethGenericSAML,
    ShibbolethSAML,
    SimpleSamlPhpSAML,
    VMwareSAML,
}

#[derive(Serialize)]
struct PostData {
    client_id: String,
    client_secret: String,
    code: String,
    grant_type: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Connection {
    // object: 'connection';
    id: String,
    organization_id: Option<String>,
    name: String,
    state: ConnectionState,
    // domains: ConnectionDomain[];
    connection_type: ConnectionType,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize, PartialEq)]
enum ConnectionState {
    Draft,
    Active,
    Inactive,
    Validating
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;
    use mockito;

    #[test]
    fn test_can_get_profile_and_access_token() {
        // Request a new server from the pool
        let mut server = mockito::Server::new();

        let mock = server.mock("POST", "/sso/token")
            .with_status(201)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "access_token": "01DMEK0J53CVMC32CK5SE0KZ8Q",
                "profile": {
                    "id": "prof_01DMC79VCBZ0NY2099737PSVF1",
                    "idp_id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
                    "connection_id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
                    "connection_type": "CasSAML",
                    "email": "alan@foo-corp.com"
                }
            }"#)
            .create();

        let workos = WorkOs::new_with_url("some-key", &server.url());
        let sso = Sso::new(workos);

        let result= sso.get_profile_and_token("01E2RJ4C05B52KKZ8FSRDAP23J", "client_123456789");
        mock.assert();

        assert!(result.is_ok());
        let response = result.unwrap();

        assert_eq!(response.access_token, "01DMEK0J53CVMC32CK5SE0KZ8Q");
        assert_eq!(response.profile.id, "prof_01DMC79VCBZ0NY2099737PSVF1");
        assert_eq!(response.profile.idp_id, "conn_01E4ZCR3C56J083X43JQXF3JK5");

        // assert_eq!(response, r#"{"access_token":"01DMEK0J53CVMC32CK5SE0KZ8Q": 101, "title": "foo", "body": "bar", "userId": 1}"#);
    }

    #[test]
    fn test_can_get_profile() {
        let mut server = mockito::Server::new();

        let mock = server.mock("GET", "/sso/profile")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "id": "prof_01DMC79VCBZ0NY2099737PSVF1",
                "idp_id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
                "connection_id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
                "connection_type": "CasSAML",
                "email": "alan@foo-corp.com"
            }"#)
            .create();

        let workos = WorkOs::new_with_url("some-key", &server.url());
        let sso = Sso::new(workos);
        let result= sso.get_profile("some_access_token");
        mock.assert();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, "prof_01DMC79VCBZ0NY2099737PSVF1");
        // ...
    }

    #[test]
    fn test_can_get_connection() {
        let mut server = mockito::Server::new();

        let mock = server.mock("GET", "/connections/some_fake_connection_id")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{
                "id": "prof_01DMC79VCBZ0NY2099737PSVF1",
                "organization_id": "conn_01E4ZCR3C56J083X43JQXF3JK5",
                "name": "connection name",
                "state": "Draft",
                "connection_type": "ADFSSAML",
                "created_at": "2024-01-01 00:00:00",
                "updated_at": "2024-01-01 00:00:00"
            }"#)
            .create();

        let mut workos = WorkOs::new_with_url("some-key", &server.url());
        workos.set_api_key("some-key");
        let sso = Sso::new(workos);
        let result= sso.get_connection("some_fake_connection_id");
        mock.assert();
        assert!(result.is_ok());

        let response = result.unwrap();
        assert_eq!(response.id, "prof_01DMC79VCBZ0NY2099737PSVF1");
        assert_eq!(response.state, ConnectionState::Draft);
    }

}
