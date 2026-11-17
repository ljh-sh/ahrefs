use std::time::Duration;

use reqwest::{Method, StatusCode};
use serde::de::DeserializeOwned;
use serde_json::Value;
use url::Url;

use crate::error::{api_error_message, Error, Result};
use crate::params::Params;

/// The production API root.
pub const DEFAULT_BASE_URL: &str = "https://api.ahrefs.com/v3";

/// Environment variable holding the API key.
pub const TOKEN_ENV: &str = "AHREFS_TOKEN";

/// A client for the Ahrefs API v3.
///
/// ```no_run
/// use ahrefs::{Client, Params};
///
/// # async fn run() -> ahrefs::Result<()> {
/// let client = Client::from_env()?;
/// let mut params = Params::new();
/// params.set("target", "example.com").set("date", "2026-11-01");
/// let rating = client.get_value("site-explorer/domain-rating", &params).await?;
/// println!("{rating}");
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    base_url: Url,
    token: Option<String>,
    max_retries: u32,
    retry_delay: Duration,
}

impl Client {
    /// A client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// A client for `token` against the production API.
    pub fn new(token: impl Into<String>) -> Result<Self> {
        ClientBuilder::default().token(token).build()
    }

    /// A client that takes its API key from the `AHREFS_TOKEN` environment
    /// variable.
    ///
    /// Public endpoints work without a key, so a client without one is still
    /// usable; it only fails once a protected endpoint is called.
    pub fn from_env() -> Result<Self> {
        let token = std::env::var(TOKEN_ENV).ok().filter(|t| !t.is_empty());
        ClientBuilder::default().token_opt(token).build()
    }

    /// The API root this client talks to.
    pub fn base_url(&self) -> &Url {
        &self.base_url
    }

    /// Whether an API key is configured.
    pub fn has_token(&self) -> bool {
        self.token.is_some()
    }

    /// Performs a `GET` and decodes the JSON body.
    pub async fn get<T: DeserializeOwned>(&self, path: &str, params: &Params) -> Result<T> {
        let value = self.get_value(path, params).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Performs a `GET` and returns the JSON body as-is.
    pub async fn get_value(&self, path: &str, params: &Params) -> Result<Value> {
        let response = self.execute(Method::GET, path, params).await?;
        let body = response.text().await?;
        Ok(serde_json::from_str(&body)?)
    }

    /// Performs a `GET` and returns the raw body, for non-JSON output formats.
    pub async fn get_raw(&self, path: &str, params: &Params) -> Result<RawResponse> {
        let response = self.execute(Method::GET, path, params).await?;
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let body = response.bytes().await?.to_vec();
        Ok(RawResponse { content_type, body })
    }

    /// Performs a `POST` with a JSON body and decodes the JSON response.
    pub async fn post<T: DeserializeOwned>(&self, path: &str, body: &Params) -> Result<T> {
        let value = self.post_value(path, body).await?;
        Ok(serde_json::from_value(value)?)
    }

    /// Performs a `POST` with a JSON body and returns the JSON response.
    pub async fn post_value(&self, path: &str, body: &Params) -> Result<Value> {
        let response = self.execute(Method::POST, path, body).await?;
        let text = response.text().await?;
        Ok(serde_json::from_str(&text)?)
    }

    /// Performs a `POST` with a JSON body and returns the raw body, for
    /// non-JSON output formats.
    pub async fn post_raw(&self, path: &str, body: &Params) -> Result<RawResponse> {
        let response = self.execute(Method::POST, path, body).await?;
        let content_type = response
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or_default()
            .to_string();
        let body = response.bytes().await?.to_vec();
        Ok(RawResponse { content_type, body })
    }

    /// Builds the absolute URL for an API path.
    fn url_for(&self, path: &str) -> Result<Url> {
        let mut url = self.base_url.clone();
        {
            let mut segments = url
                .path_segments_mut()
                .map_err(|_| Error::InvalidParam(format!("base url has no path: {path}")))?;
            segments.pop_if_empty();
            for segment in path.trim_matches('/').split('/') {
                if !segment.is_empty() {
                    segments.push(segment);
                }
            }
        }
        Ok(url)
    }

    /// Sends a request, retrying rate limits and server errors.
    async fn execute(
        &self,
        method: Method,
        path: &str,
        params: &Params,
    ) -> Result<reqwest::Response> {
        let url = self.url_for(path)?;
        let mut attempt: u32 = 0;

        loop {
            let mut request = self.http.request(method.clone(), url.clone());
            if let Some(token) = &self.token {
                request = request.bearer_auth(token);
            }
            request = match method {
                Method::GET => request.query(&params.to_query_pairs()),
                Method::POST => request.json(&params.to_json()),
                _ => request,
            };

            let response = request.send().await?;
            let status = response.status();

            if status.is_success() {
                return Ok(response);
            }

            let retryable = status == StatusCode::TOO_MANY_REQUESTS || status.is_server_error();
            if !retryable || attempt >= self.max_retries {
                let body = response.text().await.unwrap_or_default();
                return Err(api_error(status, &body));
            }

            let delay = retry_after(response.headers())
                .unwrap_or_else(|| self.retry_delay.saturating_mul(1 << attempt));
            attempt += 1;
            tokio::time::sleep(delay).await;
        }
    }
}

/// A non-JSON response body.
#[derive(Debug, Clone)]
pub struct RawResponse {
    /// The `Content-Type` the API replied with.
    pub content_type: String,
    /// The response body.
    pub body: Vec<u8>,
}

/// Builds a [`Client`].
#[derive(Debug, Clone)]
pub struct ClientBuilder {
    base_url: Url,
    token: Option<String>,
    max_retries: u32,
    retry_delay: Duration,
    timeout: Duration,
    user_agent: String,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            base_url: Url::parse(DEFAULT_BASE_URL).expect("default base url is valid"),
            token: None,
            max_retries: 3,
            retry_delay: Duration::from_secs(1),
            timeout: Duration::from_secs(300),
            user_agent: format!("ahrefs-rs/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl ClientBuilder {
    /// Sets the API root.
    pub fn base_url(mut self, base_url: impl AsRef<str>) -> Result<Self> {
        self.base_url = Url::parse(base_url.as_ref())
            .map_err(|e| Error::InvalidParam(format!("invalid base url: {e}")))?;
        Ok(self)
    }

    /// Sets the API key.
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Sets the API key, if one is available.
    pub fn token_opt(mut self, token: Option<impl Into<String>>) -> Self {
        self.token = token.map(Into::into);
        self
    }

    /// Sets how many times a rate-limited or failing request is retried.
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Sets the base delay between retries.
    pub fn retry_delay(mut self, retry_delay: Duration) -> Self {
        self.retry_delay = retry_delay;
        self
    }

    /// Sets the HTTP timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the `User-Agent` header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }

    /// Builds the client.
    pub fn build(self) -> Result<Client> {
        let http = reqwest::Client::builder()
            .timeout(self.timeout)
            .user_agent(self.user_agent)
            .build()?;
        Ok(Client {
            http,
            base_url: self.base_url,
            token: self.token,
            max_retries: self.max_retries,
            retry_delay: self.retry_delay,
        })
    }
}

/// Turns a failing response into an error.
fn api_error(status: StatusCode, body: &str) -> Error {
    let message = serde_json::from_str::<Value>(body)
        .map(|value| api_error_message(&value))
        .unwrap_or_else(|_| {
            if body.trim().is_empty() {
                status
                    .canonical_reason()
                    .unwrap_or("request failed")
                    .to_string()
            } else {
                body.trim().to_string()
            }
        });

    match status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Unauthorized {
            status: status.as_u16(),
            message,
        },
        StatusCode::TOO_MANY_REQUESTS => Error::RateLimited,
        _ => Error::Api {
            status: status.as_u16(),
            message,
        },
    }
}

/// Reads the `Retry-After` header, in seconds.
fn retry_after(headers: &reqwest::header::HeaderMap) -> Option<Duration> {
    let value = headers.get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;
    let seconds: u64 = value.trim().parse().ok()?;
    Some(Duration::from_secs(seconds))
}
