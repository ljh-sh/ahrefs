use serde_json::Value;

/// Result alias for every fallible operation in this crate.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by the Ahrefs client.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// No API key was configured.
    #[error("no API key: set AHREFS_TOKEN or pass one explicitly")]
    MissingToken,

    /// The API key was rejected or is missing.
    #[error("authentication failed (status {status}): {message}")]
    Unauthorized {
        /// The HTTP status the API replied with.
        status: u16,
        /// The message the API replied with.
        message: String,
    },

    /// The API answered with a 4xx status other than 401/403.
    #[error("api error (status {status}): {message}")]
    Api {
        /// The HTTP status the API replied with.
        status: u16,
        /// The message the API replied with.
        message: String,
    },

    /// The API kept answering 429 after every retry.
    #[error("rate limited: too many requests, retry later")]
    RateLimited,

    /// The request never reached the API.
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),

    /// The response body was not the expected shape.
    #[error("could not decode response: {0}")]
    Decode(#[from] serde_json::Error),

    /// A parameter was rejected before the request was sent.
    #[error("invalid parameter: {0}")]
    InvalidParam(String),
}

impl Error {
    /// The HTTP status the API replied with, when the failure came from the API.
    pub fn status(&self) -> Option<u16> {
        match self {
            Error::Unauthorized { status, .. } => Some(*status),
            Error::Api { status, .. } => Some(*status),
            Error::RateLimited => Some(429),
            _ => None,
        }
    }

    /// A short, stable, machine-readable tag for this error.
    pub fn kind(&self) -> &'static str {
        match self {
            Error::MissingToken => "missing_token",
            Error::Unauthorized { .. } => "unauthorized",
            Error::Api { .. } => "api_error",
            Error::RateLimited => "rate_limited",
            Error::Request(_) => "request_failed",
            Error::Decode(_) => "decode_failed",
            Error::InvalidParam(_) => "invalid_param",
        }
    }
}

/// Pulls the human-readable part out of an API error body.
///
/// The v3 API answers with `{"error": "..."}`; some edges answer with a bare
/// JSON array of strings instead.
pub fn api_error_message(body: &Value) -> String {
    match body {
        Value::Object(map) => map
            .get("error")
            .map(|v| match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .unwrap_or_else(|| body.to_string()),
        Value::String(s) => s.clone(),
        Value::Array(items) => items
            .iter()
            .map(|v| match v {
                Value::String(s) => s.clone(),
                other => other.to_string(),
            })
            .collect::<Vec<_>>()
            .join(": "),
        other => other.to_string(),
    }
}
