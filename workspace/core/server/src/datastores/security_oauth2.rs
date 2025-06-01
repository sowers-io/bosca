use crate::security::account::{Account, FacebookUser, GoogleAccount};
use async_graphql::Error;
use log::{info, warn};
use oauth2::basic::{
    BasicClient, BasicRevocationErrorResponse, BasicTokenIntrospectionResponse, BasicTokenResponse,
};
use oauth2::url::Url;
use oauth2::{reqwest, Client, EndpointMaybeSet, PkceCodeVerifier, RevocationUrl};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, EndpointNotSet, EndpointSet,
    PkceCodeChallenge, RedirectUrl, Scope, StandardRevocableToken, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityOAuth2Request {
    pub url: Url,
    pub token: CsrfToken,
    pub verifier: PkceCodeVerifier,
}

#[derive(Clone)]
pub struct SecurityOAuth2 {
    pub domain: String,
    http: reqwest::Client,
    #[allow(clippy::type_complexity)]
    clients: HashMap<
        String,
        Client<
            oauth2::basic::BasicErrorResponse,
            BasicTokenResponse,
            BasicTokenIntrospectionResponse,
            StandardRevocableToken,
            BasicRevocationErrorResponse,
            EndpointSet,
            EndpointNotSet,
            EndpointNotSet,
            EndpointMaybeSet,
            EndpointSet,
        >,
    >,
    internal_redirect_urls: Vec<String>,
}

impl SecurityOAuth2 {
    pub fn new() -> Result<Self, Error> {
        let http_client = reqwest::ClientBuilder::new()
            .redirect(reqwest::redirect::Policy::none())
            .build()?;

        let internal_redirect_urls = env::var("OAUTH2_INTERNAL_REDIRECT_URLS")
            .unwrap_or("/".to_string())
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        let redirect_url = env::var("OAUTH2_REDIRECT_URL")
            .unwrap_or("http://localhost:8000/oauth2/callback".to_string());

        let mut clients = HashMap::new();
        match env::var("OAUTH2_CLIENTS") {
            Ok(client_types) => {
                for client in client_types.split(",") {
                    let client = client.trim();
                    match client {
                        "google" => {
                            let Ok(client_id) = env::var("GOOGLE_CLIENT_ID") else {
                                return Err(Error::from(
                                    "Environment variable GOOGLE_CLIENT_ID could not be read",
                                ));
                            };
                            let Ok(client_secret) = env::var("GOOGLE_CLIENT_SECRET") else {
                                return Err(Error::from(
                                    "Environment variable GOOGLE_CLIENT_SECRET could not be read",
                                ));
                            };
                            let oauth2 = BasicClient::new(ClientId::new(client_id))
                                .set_client_secret(ClientSecret::new(client_secret))
                                .set_auth_uri(AuthUrl::new(
                                    "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
                                )?)
                                .set_token_uri(TokenUrl::new(
                                    "https://www.googleapis.com/oauth2/v3/token".to_string(),
                                )?)
                                .set_redirect_uri(RedirectUrl::new(redirect_url.clone())?)
                                .set_revocation_url_option(Some(RevocationUrl::new(
                                    "https://oauth2.googleapis.com/revoke".to_string(),
                                )?));
                            clients.insert("google".to_string(), oauth2);
                        }
                        "facebook" => {
                            let Ok(client_id) = env::var("FACEBOOK_APP_ID") else {
                                return Err(Error::from(
                                    "Environment variable FACEBOOK_APP_ID could not be read",
                                ));
                            };
                            let Ok(client_secret) = env::var("FACEBOOK_APP_SECRET") else {
                                return Err(Error::from(
                                    "Environment variable FACEBOOK_APP_SECRET could not be read",
                                ));
                            };
                            let oauth2 = BasicClient::new(ClientId::new(client_id))
                                .set_client_secret(ClientSecret::new(client_secret))
                                .set_auth_uri(AuthUrl::new(
                                    "https://www.facebook.com/v3.2/dialog/oauth".to_string(),
                                )?)
                                .set_token_uri(TokenUrl::new(
                                    "https://graph.facebook.com/v3.2/oauth/access_token"
                                        .to_string(),
                                )?)
                                .set_redirect_uri(RedirectUrl::new(redirect_url.clone())?)
                                .set_revocation_url_option(Some(RevocationUrl::new(
                                    "https://graph.facebook.com/me/permissions".to_string(),
                                )?));
                            clients.insert("facebook".to_string(), oauth2);
                        }
                        _ => {
                            let Ok(oauth2_client_id) = env::var("OAUTH2_CLIENT_ID") else {
                                return Err(Error::from(
                                    "Environment variable OAUTH2_CLIENT_ID could not be read",
                                ));
                            };
                            let Ok(oauth2_client_secret) = env::var("OAUTH2_CLIENT_SECRET") else {
                                return Err(Error::from(
                                    "Environment variable OAUTH2_CLIENT_SECRET could not be read",
                                ));
                            };
                            let Ok(oauth2_auth_url) = env::var("OAUTH2_AUTH_URL") else {
                                return Err(Error::from(
                                    "Environment variable OAUTH2_AUTH_URL could not be read",
                                ));
                            };
                            let Ok(oauth2_token_url) = env::var("OAUTH2_TOKEN_URL") else {
                                return Err(Error::from(
                                    "Environment variable OAUTH2_TOKEN_URL could not be read",
                                ));
                            };
                            let oauth2 = BasicClient::new(ClientId::new(oauth2_client_id))
                                .set_client_secret(ClientSecret::new(oauth2_client_secret))
                                .set_auth_uri(AuthUrl::new(oauth2_auth_url)?)
                                .set_token_uri(TokenUrl::new(oauth2_token_url)?)
                                .set_redirect_uri(RedirectUrl::new(redirect_url.clone())?)
                                .set_revocation_url_option(None);
                            clients.insert("custom".to_string(), oauth2);
                        }
                    }
                }
            }
            _ => {
                warn!("no auth2 clients configured")
            }
        }

        Ok(Self {
            http: http_client,
            domain: env::var("OAUTH2_DOMAIN").unwrap_or("".to_string()),
            clients,
            internal_redirect_urls,
        })
    }

    pub fn get_facebook_client_secret(&self) -> Option<String> {
        env::var("FACEBOOK_APP_SECRET").ok()
    }

    pub fn new_default_redirect_url(
        &self,
        oauth2_type: &str,
    ) -> Result<SecurityOAuth2Request, Error> {
        match oauth2_type {
            "google" => self.new_redirect_url(
                "google",
                vec![
                    Scope::new("https://www.googleapis.com/auth/userinfo.profile".to_string()),
                    Scope::new("https://www.googleapis.com/auth/userinfo.email".to_string()),
                ],
            ),
            "facebook" => self.new_redirect_url(
                "facebook",
                vec![
                    Scope::new("public_profile".to_string()),
                    Scope::new("email".to_string()),
                ],
            ),
            _ => self.new_redirect_url(
                oauth2_type,
                vec![
                    Scope::new("profile".to_string()),
                    Scope::new("email".to_string()),
                ],
            ),
        }
    }

    pub fn new_redirect_url(
        &self,
        oauth2_type: &str,
        scopes: Vec<Scope>,
    ) -> Result<SecurityOAuth2Request, Error> {
        if let Some(oauth2_client) = self.clients.get(oauth2_type) {
            let (pkce_challenge, verifier) = PkceCodeChallenge::new_random_sha256();
            let (url, token) = oauth2_client
                .authorize_url(CsrfToken::new_random)
                .add_scopes(scopes)
                .set_pkce_challenge(pkce_challenge)
                .url();
            Ok(SecurityOAuth2Request {
                url,
                token,
                verifier,
            })
        } else {
            Err(Error::from("invalid oauth2 type"))
        }
    }

    pub async fn exchange_authorization_code(
        &self,
        oauth2_type: &str,
        verifier: &str,
        authorization_code: &str,
    ) -> Result<BasicTokenResponse, Error> {
        if let Some(oauth2_client) = self.clients.get(oauth2_type) {
            let verifier: PkceCodeVerifier = PkceCodeVerifier::new(verifier.to_string());
            let token_result = oauth2_client
                .exchange_code(AuthorizationCode::new(authorization_code.to_string()))
                .set_pkce_verifier(verifier)
                .request_async(&self.http)
                .await?;
            Ok(token_result)
        } else {
            Err(Error::from("invalid oauth2 type"))
        }
    }

    pub async fn get_account(&self, oauth2_type: &str, token: &str) -> Result<Account, Error> {
        match oauth2_type {
            "google" => {
                let response = self
                    .http
                    .get("https://www.googleapis.com/oauth2/v3/userinfo")
                    .query(&[("access_token", token)])
                    .send()
                    .await?;
                let account: GoogleAccount = response.json().await?;
                if account.sub.is_empty() {
                    return Err(Error::from("invalid google account"));
                }
                if account.email.is_empty() {
                    return Err(Error::from("missing google email"));
                }
                if !account.email_verified {
                    return Err(Error::from("google account is not verified"));
                }
                Ok(Account::new_google(account))
            }
            "facebook" => {
                let response = self
                    .http
                    .get("https://graph.facebook.com/me?fields=id,name,email,picture".to_string())
                    .query(&[("access_token", token)])
                    .send()
                    .await?;
                let status = response.status();
                let account: FacebookUser = response.json().await?;
                if account.id.is_empty() {
                    info!("facebook user: {:?} -> {} -> {}", account, status, token);
                    return Err(Error::from("invalid facebook account"));
                }
                if account.email.is_empty() {
                    return Err(Error::from("missing facebook email"));
                }
                Ok(Account::new_facebook(account))
            }
            _ => Err(Error::from("invalid oauth2 type")),
        }
    }

    pub fn is_internal_redirect_url(&self, url: &str) -> bool {
        self.internal_redirect_urls
            .iter()
            .any(|internal_redirect_url| url.starts_with(internal_redirect_url))
    }
}
