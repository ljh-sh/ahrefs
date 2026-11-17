//! Site Explorer endpoints: metrics, backlinks, referring domains, top pages.

use crate::client::Client;
use crate::model::site_explorer::{
    AllBacklinksResponse, AnchorsResponse, BacklinksStatsResponse, BrokenBacklinksResponse,
    DomainRatingHistoryResponse, DomainRatingResponse, MetricsResponse, OrganicKeywordsResponse,
    OutlinksStatsResponse, RefdomainsResponse, TopPagesResponse,
};
use crate::params::{Aggregation, Mode, Params, Protocol, Report};
use crate::Result;

/// The domain rating of a target on a given date.
pub async fn domain_rating(
    client: &Client,
    target: &str,
    date: &str,
) -> Result<DomainRatingResponse> {
    let mut params = Params::new();
    params.set("target", target).set("date", date);
    client.get("site-explorer/domain-rating", &params).await
}

/// The domain rating history of a target.
pub async fn domain_rating_history(
    client: &Client,
    target: &str,
) -> Result<DomainRatingHistoryResponse> {
    let mut params = Params::new();
    params.set("target", target);
    client
        .get("site-explorer/domain-rating-history", &params)
        .await
}

/// The traffic and keyword metrics of a target on a given date.
pub async fn metrics(client: &Client, target: &str, date: &str) -> Result<MetricsResponse> {
    let mut params = Params::new();
    params.set("target", target).set("date", date);
    client.get("site-explorer/metrics", &params).await
}

/// The backlink statistics of a target on a given date.
pub async fn backlinks_stats(
    client: &Client,
    target: &str,
    date: &str,
) -> Result<BacklinksStatsResponse> {
    let mut params = Params::new();
    params.set("target", target).set("date", date);
    client.get("site-explorer/backlinks-stats", &params).await
}

/// The outlink statistics of a target on a given date.
pub async fn outlinks_stats(
    client: &Client,
    target: &str,
    date: &str,
) -> Result<OutlinksStatsResponse> {
    let mut params = Params::new();
    params.set("target", target).set("date", date);
    client.get("site-explorer/outlinks-stats", &params).await
}

/// A query for the backlink report.
#[derive(Debug, Clone, Default)]
pub struct BacklinksQuery {
    /// The domain or URL to report on.
    pub target: String,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// How the target is matched.
    pub mode: Option<Mode>,
    /// The protocol of the target.
    pub protocol: Option<Protocol>,
    /// How backlinks are grouped.
    pub aggregation: Option<Aggregation>,
    /// A time frame that adds lost backlinks, such as `live` or `all_time`.
    pub history: Option<String>,
}

/// The backlinks pointing at a target.
pub async fn backlinks(client: &Client, query: &BacklinksQuery) -> Result<AllBacklinksResponse> {
    let mut params = Params::new();
    params.set("target", query.target.as_str());
    params.set_opt("mode", query.mode.map(|m| m.as_str()));
    params.set_opt("protocol", query.protocol.map(|p| p.as_str()));
    params.set_opt("aggregation", query.aggregation.map(|a| a.as_str()));
    params.set_opt("history", query.history.clone());
    query.report.apply(&mut params);
    client.get("site-explorer/all-backlinks", &params).await
}

/// A query for the broken backlink report.
#[derive(Debug, Clone, Default)]
pub struct BrokenBacklinksQuery {
    /// The domain or URL to report on.
    pub target: String,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// How the target is matched.
    pub mode: Option<Mode>,
    /// The protocol of the target.
    pub protocol: Option<Protocol>,
}

/// The broken backlinks pointing at a target.
pub async fn broken_backlinks(
    client: &Client,
    query: &BrokenBacklinksQuery,
) -> Result<BrokenBacklinksResponse> {
    let mut params = Params::new();
    params.set("target", query.target.as_str());
    params.set_opt("mode", query.mode.map(|m| m.as_str()));
    params.set_opt("protocol", query.protocol.map(|p| p.as_str()));
    query.report.apply(&mut params);
    client.get("site-explorer/broken-backlinks", &params).await
}

/// A query for the referring domains report.
#[derive(Debug, Clone, Default)]
pub struct RefdomainsQuery {
    /// The domain or URL to report on.
    pub target: String,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// How the target is matched.
    pub mode: Option<Mode>,
    /// The protocol of the target.
    pub protocol: Option<Protocol>,
}

/// The domains linking to a target.
pub async fn refdomains(client: &Client, query: &RefdomainsQuery) -> Result<RefdomainsResponse> {
    let mut params = Params::new();
    params.set("target", query.target.as_str());
    params.set_opt("mode", query.mode.map(|m| m.as_str()));
    params.set_opt("protocol", query.protocol.map(|p| p.as_str()));
    query.report.apply(&mut params);
    client.get("site-explorer/refdomains", &params).await
}

/// A query for the anchors report.
#[derive(Debug, Clone, Default)]
pub struct AnchorsQuery {
    /// The domain or URL to report on.
    pub target: String,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// How the target is matched.
    pub mode: Option<Mode>,
    /// The protocol of the target.
    pub protocol: Option<Protocol>,
}

/// The anchors of the backlinks pointing at a target.
pub async fn anchors(client: &Client, query: &AnchorsQuery) -> Result<AnchorsResponse> {
    let mut params = Params::new();
    params.set("target", query.target.as_str());
    params.set_opt("mode", query.mode.map(|m| m.as_str()));
    params.set_opt("protocol", query.protocol.map(|p| p.as_str()));
    query.report.apply(&mut params);
    client.get("site-explorer/anchors", &params).await
}

/// A query for the organic keywords report.
#[derive(Debug, Clone, Default)]
pub struct OrganicKeywordsQuery {
    /// The domain or URL to report on.
    pub target: String,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// How the target is matched.
    pub mode: Option<Mode>,
    /// The protocol of the target.
    pub protocol: Option<Protocol>,
    /// The country the rankings are taken from.
    pub country: Option<String>,
}

/// The organic keywords a target ranks for.
pub async fn organic_keywords(
    client: &Client,
    query: &OrganicKeywordsQuery,
) -> Result<OrganicKeywordsResponse> {
    let mut params = Params::new();
    params.set("target", query.target.as_str());
    params.set_opt("mode", query.mode.map(|m| m.as_str()));
    params.set_opt("protocol", query.protocol.map(|p| p.as_str()));
    params.set_opt("country", query.country.clone());
    query.report.apply(&mut params);
    client.get("site-explorer/organic-keywords", &params).await
}

/// A query for the top pages report.
#[derive(Debug, Clone, Default)]
pub struct TopPagesQuery {
    /// The domain or URL to report on.
    pub target: String,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// How the target is matched.
    pub mode: Option<Mode>,
    /// The protocol of the target.
    pub protocol: Option<Protocol>,
    /// The country the traffic is taken from.
    pub country: Option<String>,
    /// First day of the reporting period.
    pub date_from: Option<String>,
    /// Last day of the reporting period.
    pub date_to: Option<String>,
}

/// The pages of a target that bring in the most traffic.
pub async fn top_pages(client: &Client, query: &TopPagesQuery) -> Result<TopPagesResponse> {
    let mut params = Params::new();
    params.set("target", query.target.as_str());
    params.set_opt("mode", query.mode.map(|m| m.as_str()));
    params.set_opt("protocol", query.protocol.map(|p| p.as_str()));
    params.set_opt("country", query.country.clone());
    params.set_opt("date_from", query.date_from.clone());
    params.set_opt("date_to", query.date_to.clone());
    query.report.apply(&mut params);
    client.get("site-explorer/top-pages", &params).await
}
