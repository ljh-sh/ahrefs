use serde_json::{Map, Value};

/// Ordered parameters for one API call.
///
/// The v3 API takes the same parameter names in the query string of a `GET`
/// and in the JSON body of a `POST`, so one type covers both.
#[derive(Debug, Clone, Default)]
pub struct Params {
    entries: Vec<(String, Value)>,
}

impl Params {
    /// An empty parameter set.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets a parameter, replacing an earlier value for the same key.
    pub fn set(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        let key = key.into();
        let value = value.into();
        match self.entries.iter_mut().find(|(k, _)| *k == key) {
            Some(slot) => slot.1 = value,
            None => self.entries.push((key, value)),
        }
        self
    }

    /// Sets a parameter only when the value is present.
    pub fn set_opt(
        &mut self,
        key: impl Into<String>,
        value: Option<impl Into<Value>>,
    ) -> &mut Self {
        if let Some(value) = value {
            self.set(key, value);
        }
        self
    }

    /// Appends a parameter without replacing an earlier one with the same key.
    pub fn push(&mut self, key: impl Into<String>, value: impl Into<Value>) -> &mut Self {
        self.entries.push((key.into(), value.into()));
        self
    }

    /// The value stored for `key`, if any.
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Whether any parameter is set.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// The parameters in insertion order.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.entries.iter().map(|(k, v)| (k.as_str(), v))
    }

    /// The parameters as query-string pairs.
    pub fn to_query_pairs(&self) -> Vec<(String, String)> {
        self.entries
            .iter()
            .map(|(k, v)| (k.clone(), query_value(v)))
            .collect()
    }

    /// The parameters as a JSON object, for `POST` bodies.
    pub fn to_json(&self) -> Value {
        let mut map = Map::new();
        for (key, value) in &self.entries {
            map.insert(key.clone(), value.clone());
        }
        Value::Object(map)
    }
}

/// Renders a JSON value the way the query string expects it.
///
/// Strings go out untouched, everything else is rendered compactly.
fn query_value(value: &Value) -> String {
    match value {
        Value::String(s) => s.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }
}

/// Scope of the search relative to the target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    /// The exact URL.
    Exact,
    /// Everything below the path.
    Prefix,
    /// The domain and every path on it.
    Domain,
    /// The domain and its subdomains.
    Subdomains,
}

impl Mode {
    /// The wire representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Mode::Exact => "exact",
            Mode::Prefix => "prefix",
            Mode::Domain => "domain",
            Mode::Subdomains => "subdomains",
        }
    }
}

/// Protocol the target is served over.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// Both HTTP and HTTPS.
    Both,
    /// HTTP only.
    Http,
    /// HTTPS only.
    Https,
}

impl Protocol {
    /// The wire representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Protocol::Both => "both",
            Protocol::Http => "http",
            Protocol::Https => "https",
        }
    }
}

/// Response format the API should encode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Output {
    /// JSON.
    Json,
    /// CSV.
    Csv,
    /// XML.
    Xml,
    /// PHP.
    Php,
}

impl Output {
    /// The wire representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Output::Json => "json",
            Output::Csv => "csv",
            Output::Xml => "xml",
            Output::Php => "php",
        }
    }
}

/// How backlinks are grouped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Aggregation {
    /// Group links that point at the same target.
    SimilarLinks,
    /// Keep one link per referring domain.
    OnePerDomain,
    /// Keep every link.
    All,
}

impl Aggregation {
    /// The wire representation.
    pub fn as_str(self) -> &'static str {
        match self {
            Aggregation::SimilarLinks => "similar_links",
            Aggregation::OnePerDomain => "1_per_domain",
            Aggregation::All => "all",
        }
    }
}

/// Which ranking positions to keep for a target.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetPosition {
    /// Positions 1 to 10.
    InTop10,
    /// Positions 1 to 100.
    InTop100,
}

impl TargetPosition {
    /// The wire representation.
    pub fn as_str(self) -> &'static str {
        match self {
            TargetPosition::InTop10 => "in_top10",
            TargetPosition::InTop100 => "in_top100",
        }
    }
}

/// The shared shape of every report endpoint.
///
/// `select` is mandatory on report endpoints, `where`, `order_by`, `limit` and
/// `timeout` are optional.
#[derive(Debug, Clone, Default)]
pub struct Report {
    /// Columns to return.
    pub select: Vec<String>,
    /// A filter expression built with [`crate::filter`].
    pub filter: Option<Value>,
    /// Column to sort by, `column:asc` or `column:desc`.
    pub order_by: Option<String>,
    /// Maximum number of rows.
    pub limit: Option<u32>,
    /// Server-side timeout in seconds.
    pub timeout: Option<u32>,
}

impl Report {
    /// A report that returns the given columns.
    pub fn select(columns: &[&str]) -> Self {
        Self {
            select: columns.iter().map(|c| c.to_string()).collect(),
            ..Self::default()
        }
    }

    /// Writes this report into a parameter set.
    pub fn apply(&self, params: &mut Params) {
        if !self.select.is_empty() {
            params.set("select", self.select.join(","));
        }
        params.set_opt("where", self.filter.clone());
        params.set_opt("order_by", self.order_by.clone());
        params.set_opt("limit", self.limit.map(Value::from));
        params.set_opt("timeout", self.timeout.map(Value::from));
    }
}
