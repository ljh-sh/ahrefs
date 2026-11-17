//! Response models for the `keywords-explorer` API group.

#![allow(missing_docs)]

use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "cps")]
    pub cps: Option<f64>,
    #[serde(default, rename = "difficulty")]
    pub difficulty: Option<i64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "global_traffic_potential")]
    pub global_traffic_potential: Option<i64>,
    #[serde(default, rename = "global_volume")]
    pub global_volume: Option<i64>,
    #[serde(default, rename = "intents")]
    pub intents: Option<Value>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "parent_topic")]
    pub parent_topic: Option<String>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_last_update")]
    pub serp_last_update: Option<String>,
    #[serde(default, rename = "traffic_potential")]
    pub traffic_potential: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "volume_desktop_pct")]
    pub volume_desktop_pct: Option<f64>,
    #[serde(default, rename = "volume_mobile_pct")]
    pub volume_mobile_pct: Option<f64>,
    #[serde(default, rename = "volume_monthly")]
    pub volume_monthly: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MatchingTermsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem2 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "cps")]
    pub cps: Option<f64>,
    #[serde(default, rename = "difficulty")]
    pub difficulty: Option<i64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "global_traffic_potential")]
    pub global_traffic_potential: Option<i64>,
    #[serde(default, rename = "global_volume")]
    pub global_volume: Option<i64>,
    #[serde(default, rename = "intents")]
    pub intents: Option<Value>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "parent_topic")]
    pub parent_topic: Option<String>,
    #[serde(default, rename = "parent_volume")]
    pub parent_volume: Option<i64>,
    #[serde(default, rename = "searches_pct_clicks_organic_and_paid")]
    pub searches_pct_clicks_organic_and_paid: Option<f64>,
    #[serde(default, rename = "searches_pct_clicks_organic_only")]
    pub searches_pct_clicks_organic_only: Option<f64>,
    #[serde(default, rename = "searches_pct_clicks_paid_only")]
    pub searches_pct_clicks_paid_only: Option<f64>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_last_update")]
    pub serp_last_update: Option<String>,
    #[serde(default, rename = "traffic_potential")]
    pub traffic_potential: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "volume_desktop_pct")]
    pub volume_desktop_pct: Option<f64>,
    #[serde(default, rename = "volume_mobile_pct")]
    pub volume_mobile_pct: Option<f64>,
    #[serde(default, rename = "volume_monthly")]
    pub volume_monthly: Option<i64>,
    #[serde(default, rename = "volume_monthly_history")]
    pub volume_monthly_history: Option<Vec<Value>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OverviewResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem3 {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "cps")]
    pub cps: Option<f64>,
    #[serde(default, rename = "difficulty")]
    pub difficulty: Option<i64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "global_traffic_potential")]
    pub global_traffic_potential: Option<i64>,
    #[serde(default, rename = "global_volume")]
    pub global_volume: Option<i64>,
    #[serde(default, rename = "intents")]
    pub intents: Option<Value>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "parent_topic")]
    pub parent_topic: Option<String>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_last_update")]
    pub serp_last_update: Option<String>,
    #[serde(default, rename = "traffic_potential")]
    pub traffic_potential: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "volume_desktop_pct")]
    pub volume_desktop_pct: Option<f64>,
    #[serde(default, rename = "volume_mobile_pct")]
    pub volume_mobile_pct: Option<f64>,
    #[serde(default, rename = "volume_monthly")]
    pub volume_monthly: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RelatedTermsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem4 {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "cps")]
    pub cps: Option<f64>,
    #[serde(default, rename = "difficulty")]
    pub difficulty: Option<i64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "global_traffic_potential")]
    pub global_traffic_potential: Option<i64>,
    #[serde(default, rename = "global_volume")]
    pub global_volume: Option<i64>,
    #[serde(default, rename = "intents")]
    pub intents: Option<Value>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "parent_topic")]
    pub parent_topic: Option<String>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_last_update")]
    pub serp_last_update: Option<String>,
    #[serde(default, rename = "traffic_potential")]
    pub traffic_potential: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "volume_desktop_pct")]
    pub volume_desktop_pct: Option<f64>,
    #[serde(default, rename = "volume_mobile_pct")]
    pub volume_mobile_pct: Option<f64>,
    #[serde(default, rename = "volume_monthly")]
    pub volume_monthly: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SearchSuggestionsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem4>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EventsItem {
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "message")]
    pub message: Option<String>,
    #[serde(default, rename = "details")]
    pub details: Option<String>,
    #[serde(default, rename = "happened_at")]
    pub happened_at: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NoticesItem {
    #[serde(default, rename = "events")]
    pub events: Option<Vec<EventsItem>>,
    #[serde(default, rename = "notice_id")]
    pub notice_id: Option<String>,
    #[serde(default, rename = "visible_since")]
    pub visible_since: Option<String>,
    #[serde(default, rename = "visible_until")]
    pub visible_until: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ServiceNoticesResponse {
    #[serde(default, rename = "notices")]
    pub notices: Option<Vec<NoticesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VolumeMonthlyHistoryItem {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CountriesItem {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "difficulty")]
    pub difficulty: Option<i64>,
    #[serde(default, rename = "growth_rate_12m")]
    pub growth_rate_12m: Option<f64>,
    #[serde(default, rename = "has_serp")]
    pub has_serp: Option<bool>,
    #[serde(default, rename = "top_keyword")]
    pub top_keyword: Option<String>,
    #[serde(default, rename = "top_page")]
    pub top_page: Option<String>,
    #[serde(default, rename = "top_page_title")]
    pub top_page_title: Option<String>,
    #[serde(default, rename = "traffic_potential")]
    pub traffic_potential: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "volume_monthly_history")]
    pub volume_monthly_history: Option<Vec<VolumeMonthlyHistoryItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VolumeByCountryResponse {
    #[serde(default, rename = "countries")]
    pub countries: Option<Vec<CountriesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VolumeHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem>>,
}
