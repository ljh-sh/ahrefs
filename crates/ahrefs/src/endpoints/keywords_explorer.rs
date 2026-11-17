//! Keywords Explorer endpoints: metrics for keywords, ideas and history.

use crate::client::Client;
use crate::model::keywords_explorer::{
    MatchingTermsResponse, OverviewResponse, RelatedTermsResponse, SearchSuggestionsResponse,
    VolumeByCountryResponse, VolumeHistoryResponse,
};
use crate::params::{Mode, Params, Report, TargetPosition};
use crate::Result;

/// A query for the keyword metrics report.
#[derive(Debug, Clone, Default)]
pub struct OverviewQuery {
    /// Keywords to report on, comma separated.
    pub keywords: Option<String>,
    /// An existing keyword list to report on.
    pub keyword_list_id: Option<u64>,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// The country to report on, `all` for every location.
    pub country: Option<String>,
    /// A domain or URL whose rankings narrow the report down.
    pub target: Option<String>,
    /// How the target is matched.
    pub mode: Option<Mode>,
    /// Which ranking positions to keep.
    pub target_position: Option<TargetPosition>,
    /// First month of the volume history.
    pub volume_monthly_date_from: Option<String>,
    /// Last month of the volume history.
    pub volume_monthly_date_to: Option<String>,
}

/// The metrics of one or more keywords.
pub async fn overview(client: &Client, query: &OverviewQuery) -> Result<OverviewResponse> {
    let mut params = Params::new();
    params.set_opt("keywords", query.keywords.clone());
    params.set_opt(
        "keyword_list_id",
        query.keyword_list_id.map(|v| v.to_string()),
    );
    params.set_opt("country", query.country.clone());
    params.set_opt("target", query.target.clone());
    params.set_opt("mode", query.mode.map(|m| m.as_str()));
    params.set_opt("target_position", query.target_position.map(|p| p.as_str()));
    params.set_opt(
        "volume_monthly_date_from",
        query.volume_monthly_date_from.clone(),
    );
    params.set_opt(
        "volume_monthly_date_to",
        query.volume_monthly_date_to.clone(),
    );
    query.report.apply(&mut params);
    client.get("keywords-explorer/overview", &params).await
}

/// A query for the search volume history report.
#[derive(Debug, Clone, Default)]
pub struct VolumeHistoryQuery {
    /// Keywords to report on, comma separated.
    pub keywords: Option<String>,
    /// An existing keyword list to report on.
    pub keyword_list_id: Option<u64>,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// The country to report on, `all` for every location.
    pub country: Option<String>,
}

/// The monthly search volume of one or more keywords.
pub async fn volume_history(
    client: &Client,
    query: &VolumeHistoryQuery,
) -> Result<VolumeHistoryResponse> {
    let mut params = Params::new();
    params.set_opt("keywords", query.keywords.clone());
    params.set_opt(
        "keyword_list_id",
        query.keyword_list_id.map(|v| v.to_string()),
    );
    params.set_opt("country", query.country.clone());
    query.report.apply(&mut params);
    client
        .get("keywords-explorer/volume-history", &params)
        .await
}

/// A query for the matching terms report.
#[derive(Debug, Clone, Default)]
pub struct MatchingTermsQuery {
    /// The keywords to expand, comma separated.
    pub keywords: Option<String>,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// The country to report on, `all` for every location.
    pub country: Option<String>,
}

/// The keywords that match the given keywords.
pub async fn matching_terms(
    client: &Client,
    query: &MatchingTermsQuery,
) -> Result<MatchingTermsResponse> {
    let mut params = Params::new();
    params.set_opt("keywords", query.keywords.clone());
    params.set_opt("country", query.country.clone());
    query.report.apply(&mut params);
    client
        .get("keywords-explorer/matching-terms", &params)
        .await
}

/// A query for the related terms report.
#[derive(Debug, Clone, Default)]
pub struct RelatedTermsQuery {
    /// The keywords to expand, comma separated.
    pub keywords: Option<String>,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// The country to report on, `all` for every location.
    pub country: Option<String>,
}

/// The keywords related to the given keywords.
pub async fn related_terms(
    client: &Client,
    query: &RelatedTermsQuery,
) -> Result<RelatedTermsResponse> {
    let mut params = Params::new();
    params.set_opt("keywords", query.keywords.clone());
    params.set_opt("country", query.country.clone());
    query.report.apply(&mut params);
    client.get("keywords-explorer/related-terms", &params).await
}

/// A query for the search suggestions report.
#[derive(Debug, Clone, Default)]
pub struct SearchSuggestionsQuery {
    /// The keywords to expand, comma separated.
    pub keywords: Option<String>,
    /// Columns to return and how to narrow them down.
    pub report: Report,
    /// The country to report on, `all` for every location.
    pub country: Option<String>,
}

/// The search suggestions for the given keywords.
pub async fn search_suggestions(
    client: &Client,
    query: &SearchSuggestionsQuery,
) -> Result<SearchSuggestionsResponse> {
    let mut params = Params::new();
    params.set_opt("keywords", query.keywords.clone());
    params.set_opt("country", query.country.clone());
    query.report.apply(&mut params);
    client
        .get("keywords-explorer/search-suggestions", &params)
        .await
}

/// A query for the search volume by country report.
#[derive(Debug, Clone, Default)]
pub struct VolumeByCountryQuery {
    /// The keywords to report on, comma separated.
    pub keywords: Option<String>,
    /// Columns to return and how to narrow them down.
    pub report: Report,
}

/// The search volume of keywords split by country.
pub async fn volume_by_country(
    client: &Client,
    query: &VolumeByCountryQuery,
) -> Result<VolumeByCountryResponse> {
    let mut params = Params::new();
    params.set_opt("keywords", query.keywords.clone());
    query.report.apply(&mut params);
    client
        .get("keywords-explorer/volume-by-country", &params)
        .await
}
