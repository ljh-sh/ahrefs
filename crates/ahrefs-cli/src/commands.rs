use std::time::Duration;

use ahrefs::{Client, Params};
use serde_json::{json, Value};

use crate::cli::{
    Aggregation, Cli, Command, Format, GlobalArgs, KeywordsExplorerCommand, Method, Mode, Protocol,
    PublicCommand, SiteExplorerCommand, TargetPosition,
};
use crate::output::{self, Payload};

/// Result of a command.
pub type Result<T> = std::result::Result<T, CliError>;

/// A failure that is reported to the user as JSON.
#[derive(Debug)]
pub struct CliError {
    /// Stable, machine readable tag.
    pub kind: &'static str,
    /// What went wrong.
    pub message: String,
    /// The HTTP status, when the API produced one.
    pub status: Option<u16>,
}

impl CliError {
    fn new(kind: &'static str, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
            status: None,
        }
    }
}

impl From<ahrefs::Error> for CliError {
    fn from(err: ahrefs::Error) -> Self {
        Self {
            kind: err.kind(),
            message: err.to_string(),
            status: err.status(),
        }
    }
}

const BACKLINKS_SELECT: &[&str] = &[
    "url_from",
    "url_to",
    "anchor",
    "domain_rating_source",
    "first_seen",
    "last_seen",
    "is_dofollow",
    "is_lost",
];

const BROKEN_BACKLINKS_SELECT: &[&str] = &[
    "url_from",
    "url_to",
    "anchor",
    "broken_redirect_reason",
    "broken_redirect_new_target",
    "first_seen",
    "last_seen",
    "http_code",
];

const REFDOMAINS_SELECT: &[&str] = &[
    "domain",
    "domain_rating",
    "links_to_target",
    "dofollow_links",
    "first_seen",
    "last_seen",
    "traffic_domain",
];

const ANCHORS_SELECT: &[&str] = &[
    "anchor",
    "dofollow_links",
    "links_to_target",
    "refdomains",
    "first_seen",
    "last_seen",
];

const ORGANIC_KEYWORDS_SELECT: &[&str] = &[
    "keyword",
    "best_position",
    "best_position_url",
    "volume",
    "cpc",
    "keyword_difficulty",
];

const TOP_PAGES_SELECT: &[&str] = &[
    "url",
    "sum_traffic",
    "top_keyword",
    "referring_domains",
    "page_type",
];

const KEYWORD_SELECT: &[&str] = &["keyword", "volume", "cpc", "difficulty", "country"];

/// Runs the parsed command line and returns what should be printed.
pub async fn run(cli: Cli) -> Result<String> {
    let format = cli.global.format;
    let client = build_client(&cli.global)?;
    let payload = match cli.command {
        Command::SiteExplorer(command) => site_explorer(&client, format, command).await?,
        Command::KeywordsExplorer(command) => keywords_explorer(&client, format, command).await?,
        Command::Public(command) => public(&client, format, command).await?,
        Command::Raw {
            path,
            method,
            params: raw_params,
        } => raw(&client, format, &path, method, &raw_params).await?,
    };

    Ok(output::render(format, &payload))
}

/// Builds a client from the global options.
fn build_client(global: &GlobalArgs) -> Result<Client> {
    Client::builder()
        .base_url(&global.base_url)?
        .token_opt(global.token.clone())
        .max_retries(global.retries)
        .timeout(Duration::from_secs(global.timeout as u64))
        .user_agent(format!("ahrefs-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()
        .map_err(CliError::from)
}

/// Performs a `GET` and returns the payload in the requested shape.
async fn fetch(client: &Client, format: Format, path: &str, params: &Params) -> Result<Payload> {
    if format == Format::Raw {
        let response = client.get_raw(path, params).await?;
        return Ok(Payload::Raw(response.body));
    }
    let value = client.get_value(path, params).await?;
    Ok(Payload::Json(value))
}

/// Performs a `POST` and returns the payload in the requested shape.
async fn fetch_post(client: &Client, format: Format, path: &str, body: &Params) -> Result<Payload> {
    if format == Format::Raw {
        let response = client.post_raw(path, body).await?;
        return Ok(Payload::Raw(response.body));
    }
    let value = client.post_value(path, body).await?;
    Ok(Payload::Json(value))
}

/// Runs a Site Explorer command.
async fn site_explorer(
    client: &Client,
    format: Format,
    command: SiteExplorerCommand,
) -> Result<Payload> {
    let mut params = Params::new();
    let path = match command {
        SiteExplorerCommand::DomainRating { target, date } => {
            params.set("target", target).set("date", date);
            "site-explorer/domain-rating"
        }
        SiteExplorerCommand::DomainRatingHistory { target, select } => {
            params.set("target", target);
            params.set("select", columns(&select, &["date", "domain_rating"]));
            "site-explorer/domain-rating-history"
        }
        SiteExplorerCommand::Metrics { target, date } => {
            params.set("target", target).set("date", date);
            "site-explorer/metrics"
        }
        SiteExplorerCommand::BacklinksStats { target, date } => {
            params.set("target", target).set("date", date);
            "site-explorer/backlinks-stats"
        }
        SiteExplorerCommand::OutlinksStats { target, date } => {
            params.set("target", target).set("date", date);
            "site-explorer/outlinks-stats"
        }
        SiteExplorerCommand::Backlinks {
            target,
            select,
            filter,
            order_by,
            limit,
            mode,
            protocol,
            aggregation,
            history,
        } => {
            params.set("target", target);
            report(
                &mut params,
                &select,
                &filter,
                &order_by,
                limit,
                BACKLINKS_SELECT,
            )?;
            scope(&mut params, mode, protocol);
            params.set_opt(
                "aggregation",
                aggregation.map(|a| match a {
                    Aggregation::SimilarLinks => "similar_links",
                    Aggregation::OnePerDomain => "1_per_domain",
                    Aggregation::All => "all",
                }),
            );
            params.set_opt("history", history);
            "site-explorer/all-backlinks"
        }
        SiteExplorerCommand::BrokenBacklinks {
            target,
            select,
            filter,
            order_by,
            limit,
            mode,
            protocol,
        } => {
            params.set("target", target);
            report(
                &mut params,
                &select,
                &filter,
                &order_by,
                limit,
                BROKEN_BACKLINKS_SELECT,
            )?;
            scope(&mut params, mode, protocol);
            "site-explorer/broken-backlinks"
        }
        SiteExplorerCommand::Refdomains {
            target,
            select,
            filter,
            order_by,
            limit,
            mode,
            protocol,
        } => {
            params.set("target", target);
            report(
                &mut params,
                &select,
                &filter,
                &order_by,
                limit,
                REFDOMAINS_SELECT,
            )?;
            scope(&mut params, mode, protocol);
            "site-explorer/refdomains"
        }
        SiteExplorerCommand::Anchors {
            target,
            select,
            filter,
            order_by,
            limit,
            mode,
            protocol,
        } => {
            params.set("target", target);
            report(
                &mut params,
                &select,
                &filter,
                &order_by,
                limit,
                ANCHORS_SELECT,
            )?;
            scope(&mut params, mode, protocol);
            "site-explorer/anchors"
        }
        SiteExplorerCommand::OrganicKeywords {
            target,
            select,
            filter,
            order_by,
            limit,
            mode,
            protocol,
            country,
        } => {
            params.set("target", target);
            report(
                &mut params,
                &select,
                &filter,
                &order_by,
                limit,
                ORGANIC_KEYWORDS_SELECT,
            )?;
            scope(&mut params, mode, protocol);
            params.set_opt("country", country);
            "site-explorer/organic-keywords"
        }
        SiteExplorerCommand::TopPages {
            target,
            select,
            filter,
            order_by,
            limit,
            mode,
            protocol,
            country,
            date_from,
            date_to,
        } => {
            params.set("target", target);
            report(
                &mut params,
                &select,
                &filter,
                &order_by,
                limit,
                TOP_PAGES_SELECT,
            )?;
            scope(&mut params, mode, protocol);
            params.set_opt("country", country);
            params.set_opt("date_from", date_from);
            params.set_opt("date_to", date_to);
            "site-explorer/top-pages"
        }
    };

    fetch(client, format, path, &params).await
}

/// Runs a Keywords Explorer command.
async fn keywords_explorer(
    client: &Client,
    format: Format,
    command: KeywordsExplorerCommand,
) -> Result<Payload> {
    let mut params = Params::new();
    let path = match command {
        KeywordsExplorerCommand::Overview {
            keywords,
            keyword_list_id,
            select,
            filter,
            order_by,
            limit,
            country,
            target,
            mode,
            target_position,
        } => {
            params.set_opt("keywords", keywords);
            params.set_opt("keyword_list_id", keyword_list_id.map(|v| v.to_string()));
            params.set("country", country.unwrap_or_else(|| "us".to_string()));
            params.set_opt("target", target);
            scope(&mut params, mode, None);
            params.set_opt(
                "target_position",
                target_position.map(|p| match p {
                    TargetPosition::InTop10 => "in_top10",
                    TargetPosition::InTop100 => "in_top100",
                }),
            );
            report(
                &mut params,
                &select,
                &filter,
                &order_by,
                limit,
                KEYWORD_SELECT,
            )?;
            "keywords-explorer/overview"
        }
        KeywordsExplorerCommand::VolumeHistory {
            keywords,
            select,
            limit,
            country,
        } => {
            params.set_opt("keywords", keywords);
            params.set("country", country.unwrap_or_else(|| "us".to_string()));
            report(
                &mut params,
                &select,
                &None,
                &None,
                limit,
                &["date", "volume"],
            )?;
            "keywords-explorer/volume-history"
        }
        KeywordsExplorerCommand::MatchingTerms {
            keywords,
            select,
            limit,
            country,
        } => {
            params.set("keywords", keywords);
            params.set("country", country.unwrap_or_else(|| "us".to_string()));
            report(&mut params, &select, &None, &None, limit, KEYWORD_SELECT)?;
            "keywords-explorer/matching-terms"
        }
        KeywordsExplorerCommand::RelatedTerms {
            keywords,
            select,
            limit,
            country,
        } => {
            params.set("keywords", keywords);
            params.set("country", country.unwrap_or_else(|| "us".to_string()));
            report(&mut params, &select, &None, &None, limit, KEYWORD_SELECT)?;
            "keywords-explorer/related-terms"
        }
        KeywordsExplorerCommand::SearchSuggestions {
            keywords,
            select,
            limit,
            country,
        } => {
            params.set("keywords", keywords);
            params.set("country", country.unwrap_or_else(|| "us".to_string()));
            report(&mut params, &select, &None, &None, limit, KEYWORD_SELECT)?;
            "keywords-explorer/search-suggestions"
        }
        KeywordsExplorerCommand::VolumeByCountry {
            keywords,
            select,
            limit,
        } => {
            params.set("keywords", keywords);
            report(
                &mut params,
                &select,
                &None,
                &None,
                limit,
                &["country", "volume", "top_keyword"],
            )?;
            "keywords-explorer/volume-by-country"
        }
    };

    fetch(client, format, path, &params).await
}

/// Runs a public command.
async fn public(client: &Client, format: Format, command: PublicCommand) -> Result<Payload> {
    let mut params = Params::new();
    let path = match command {
        PublicCommand::CrawlerIps => "public/crawler-ips",
        PublicCommand::CrawlerIpRanges => "public/crawler-ip-ranges",
        PublicCommand::DomainRatingFree { target } => {
            params.set("target", target);
            "public/domain-rating-free"
        }
        PublicCommand::DomainRatingTopDomains => "public/domain-rating-top-domains",
    };

    fetch(client, format, path, &params).await
}

/// Runs a raw endpoint call.
async fn raw(
    client: &Client,
    format: Format,
    path: &str,
    method: Method,
    raw_params: &[String],
) -> Result<Payload> {
    let mut params = Params::new();
    for pair in raw_params {
        let (key, value) = pair.split_once('=').ok_or_else(|| {
            CliError::new("invalid_param", format!("expected key=value, got `{pair}`"))
        })?;
        params.push(key, value);
    }

    match method {
        Method::Get => fetch(client, format, path, &params).await,
        Method::Post => fetch_post(client, format, path, &params).await,
    }
}

/// The columns to ask for.
fn columns(select: &Option<Vec<String>>, defaults: &[&str]) -> String {
    match select {
        Some(list) => list.join(","),
        None => defaults.join(","),
    }
}

/// Adds the shared report parameters.
fn report(
    params: &mut Params,
    select: &Option<Vec<String>>,
    filter: &Option<String>,
    order_by: &Option<String>,
    limit: Option<u32>,
    defaults: &[&str],
) -> Result<()> {
    params.set("select", columns(select, defaults));
    if let Some(text) = filter {
        params.set("where", parse_filter(text)?);
    }
    params.set_opt("order_by", order_by.clone());
    params.set_opt("limit", limit.map(Value::from));
    Ok(())
}

/// Adds the shared scope parameters.
fn scope(params: &mut Params, mode: Option<Mode>, protocol: Option<Protocol>) {
    params.set_opt(
        "mode",
        mode.map(|m| match m {
            Mode::Exact => "exact",
            Mode::Prefix => "prefix",
            Mode::Domain => "domain",
            Mode::Subdomains => "subdomains",
        }),
    );
    params.set_opt(
        "protocol",
        protocol.map(|p| match p {
            Protocol::Both => "both",
            Protocol::Http => "http",
            Protocol::Https => "https",
        }),
    );
}

/// Turns a filter expression into the JSON the API expects.
///
/// A JSON object is passed through, `field=value` is shorthand for an equality
/// check.
fn parse_filter(text: &str) -> Result<Value> {
    let trimmed = text.trim();
    if trimmed.starts_with('{') {
        return serde_json::from_str(trimmed)
            .map_err(|e| CliError::new("invalid_param", format!("invalid filter JSON: {e}")));
    }

    for (symbol, operator) in [
        ("!=", "neq"),
        (">=", "gte"),
        ("<=", "lte"),
        ("=", "eq"),
        (">", "gt"),
        ("<", "lt"),
    ] {
        if let Some((field, value)) = trimmed.split_once(symbol) {
            let field = field.trim();
            if field.is_empty() {
                return Err(CliError::new(
                    "invalid_param",
                    format!("filter `{text}` has no field name"),
                ));
            }
            return Ok(json!({
                "field": field,
                "is": [operator, literal(value.trim())],
            }));
        }
    }

    Err(CliError::new(
        "invalid_param",
        format!("filter `{text}` is neither JSON nor field=value"),
    ))
}

/// Reads a filter value as a number, a boolean or a string.
fn literal(text: &str) -> Value {
    if let Ok(number) = text.parse::<i64>() {
        return Value::from(number);
    }
    if let Ok(number) = text.parse::<f64>() {
        return Value::from(number);
    }
    match text {
        "true" => Value::Bool(true),
        "false" => Value::Bool(false),
        other => Value::String(other.to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn json_filters_are_passed_through() {
        let parsed = parse_filter(r#"{"field":"foo","is":["eq",3]}"#).unwrap();
        assert_eq!(parsed["field"], json!("foo"));
        assert_eq!(parsed["is"][0], json!("eq"));
    }

    #[test]
    fn shorthand_filters_become_equality_checks() {
        let parsed = parse_filter("domain_rating>50").unwrap();
        assert_eq!(parsed["field"], json!("domain_rating"));
        assert_eq!(parsed["is"][0], json!("gt"));
        assert_eq!(parsed["is"][1], json!(50));
    }

    #[test]
    fn shorthand_filters_keep_strings_as_strings() {
        let parsed = parse_filter("domain=example.com").unwrap();
        assert_eq!(parsed["is"][1], json!("example.com"));
    }

    #[test]
    fn broken_filters_are_rejected() {
        assert!(parse_filter("nonsense").is_err());
    }
}
