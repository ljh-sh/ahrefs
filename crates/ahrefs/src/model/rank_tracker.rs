//! Response models for the `rank-tracker` API group.

#![allow(missing_docs)]

use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsDomainsItem {
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "keywords")]
    pub keywords: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
    #[serde(default, rename = "share_of_traffic_value")]
    pub share_of_traffic_value: Option<f64>,
    #[serde(default, rename = "share_of_traffic_value_prev")]
    pub share_of_traffic_value_prev: Option<f64>,
    #[serde(default, rename = "share_of_voice")]
    pub share_of_voice: Option<f64>,
    #[serde(default, rename = "share_of_voice_prev")]
    pub share_of_voice_prev: Option<f64>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "traffic_prev")]
    pub traffic_prev: Option<i64>,
    #[serde(default, rename = "traffic_value")]
    pub traffic_value: Option<i64>,
    #[serde(default, rename = "traffic_value_prev")]
    pub traffic_value_prev: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsDomainsResponse {
    #[serde(default, rename = "competitors-domains")]
    pub competitors_domains: Option<Vec<CompetitorsDomainsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem {
    #[serde(default, rename = "competitors_list")]
    pub competitors_list: Option<Vec<Value>>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "keyword_difficulty")]
    pub keyword_difficulty: Option<i64>,
    #[serde(default, rename = "keyword_has_data")]
    pub keyword_has_data: Option<bool>,
    #[serde(default, rename = "keyword_is_frozen")]
    pub keyword_is_frozen: Option<bool>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_updated")]
    pub serp_updated: Option<String>,
    #[serde(default, rename = "serp_updated_prev")]
    pub serp_updated_prev: Option<String>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsOverviewResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsPagesItem {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<i64>,
    #[serde(default, rename = "share_of_traffic_value")]
    pub share_of_traffic_value: Option<f64>,
    #[serde(default, rename = "share_of_traffic_value_prev")]
    pub share_of_traffic_value_prev: Option<f64>,
    #[serde(default, rename = "share_of_voice")]
    pub share_of_voice: Option<f64>,
    #[serde(default, rename = "share_of_voice_prev")]
    pub share_of_voice_prev: Option<f64>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "title_prev")]
    pub title_prev: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "traffic_prev")]
    pub traffic_prev: Option<i64>,
    #[serde(default, rename = "traffic_value")]
    pub traffic_value: Option<i64>,
    #[serde(default, rename = "traffic_value_prev")]
    pub traffic_value_prev: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsPagesResponse {
    #[serde(default, rename = "competitors-pages")]
    pub competitors_pages: Option<Vec<CompetitorsPagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsMetricsItem {
    #[serde(default, rename = "ai_overview_count")]
    pub ai_overview_count: Option<i64>,
    #[serde(default, rename = "average_position")]
    pub average_position: Option<f64>,
    #[serde(default, rename = "competitor")]
    pub competitor: Option<String>,
    #[serde(default, rename = "discussions_count")]
    pub discussions_count: Option<i64>,
    #[serde(default, rename = "featured_snippet_count")]
    pub featured_snippet_count: Option<i64>,
    #[serde(default, rename = "image_pack_count")]
    pub image_pack_count: Option<i64>,
    #[serde(default, rename = "knowledge_card_count")]
    pub knowledge_card_count: Option<i64>,
    #[serde(default, rename = "knowledge_panel_count")]
    pub knowledge_panel_count: Option<i64>,
    #[serde(default, rename = "pos_1_3")]
    pub pos_1_3: Option<i64>,
    #[serde(default, rename = "pos_4_10")]
    pub pos_4_10: Option<i64>,
    #[serde(default, rename = "pos_11_20")]
    pub pos_11_20: Option<i64>,
    #[serde(default, rename = "pos_21_50")]
    pub pos_21_50: Option<i64>,
    #[serde(default, rename = "pos_51_plus")]
    pub pos_51_plus: Option<i64>,
    #[serde(default, rename = "pos_no_rank")]
    pub pos_no_rank: Option<i64>,
    #[serde(default, rename = "share_of_traffic_value")]
    pub share_of_traffic_value: Option<f64>,
    #[serde(default, rename = "share_of_voice")]
    pub share_of_voice: Option<f64>,
    #[serde(default, rename = "sitelinks_count")]
    pub sitelinks_count: Option<i64>,
    #[serde(default, rename = "thumbnail_count")]
    pub thumbnail_count: Option<i64>,
    #[serde(default, rename = "top_stories_count")]
    pub top_stories_count: Option<i64>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "traffic_value")]
    pub traffic_value: Option<i64>,
    #[serde(default, rename = "video_preview_count")]
    pub video_preview_count: Option<i64>,
    #[serde(default, rename = "videos_count")]
    pub videos_count: Option<i64>,
    #[serde(default, rename = "x_count")]
    pub x_count: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsStatsResponse {
    #[serde(default, rename = "competitors-metrics")]
    pub competitors_metrics: Option<Vec<CompetitorsMetricsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OverviewsItem {
    #[serde(default, rename = "best_position_has_thumbnail")]
    pub best_position_has_thumbnail: Option<bool>,
    #[serde(default, rename = "best_position_has_thumbnail_previous")]
    pub best_position_has_thumbnail_previous: Option<bool>,
    #[serde(default, rename = "best_position_has_video_preview")]
    pub best_position_has_video_preview: Option<bool>,
    #[serde(default, rename = "best_position_has_video_preview_previous")]
    pub best_position_has_video_preview_previous: Option<bool>,
    #[serde(default, rename = "best_position_kind")]
    pub best_position_kind: Option<String>,
    #[serde(default, rename = "best_position_kind_previous")]
    pub best_position_kind_previous: Option<String>,
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "clicks_per_search")]
    pub clicks_per_search: Option<f64>,
    #[serde(default, rename = "cost_per_click")]
    pub cost_per_click: Option<i64>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "country_prev")]
    pub country_prev: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "is_branded")]
    pub is_branded: Option<bool>,
    #[serde(default, rename = "is_commercial")]
    pub is_commercial: Option<bool>,
    #[serde(default, rename = "is_informational")]
    pub is_informational: Option<bool>,
    #[serde(default, rename = "is_local")]
    pub is_local: Option<bool>,
    #[serde(default, rename = "is_navigational")]
    pub is_navigational: Option<bool>,
    #[serde(default, rename = "is_transactional")]
    pub is_transactional: Option<bool>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "keyword_difficulty")]
    pub keyword_difficulty: Option<i64>,
    #[serde(default, rename = "keyword_has_data")]
    pub keyword_has_data: Option<bool>,
    #[serde(default, rename = "keyword_is_frozen")]
    pub keyword_is_frozen: Option<bool>,
    #[serde(default, rename = "keyword_prev")]
    pub keyword_prev: Option<String>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "language_prev")]
    pub language_prev: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "location_prev")]
    pub location_prev: Option<String>,
    #[serde(default, rename = "parent_topic")]
    pub parent_topic: Option<String>,
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "position_diff")]
    pub position_diff: Option<i64>,
    #[serde(default, rename = "position_prev")]
    pub position_prev: Option<i64>,
    #[serde(default, rename = "search_type_image")]
    pub search_type_image: Option<f64>,
    #[serde(default, rename = "search_type_news")]
    pub search_type_news: Option<f64>,
    #[serde(default, rename = "search_type_video")]
    pub search_type_video: Option<f64>,
    #[serde(default, rename = "search_type_web")]
    pub search_type_web: Option<f64>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_features_prev")]
    pub serp_features_prev: Option<Vec<String>>,
    #[serde(default, rename = "serp_updated")]
    pub serp_updated: Option<String>,
    #[serde(default, rename = "serp_updated_prev")]
    pub serp_updated_prev: Option<String>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(default, rename = "tags_prev")]
    pub tags_prev: Option<Vec<String>>,
    #[serde(default, rename = "target_positions_count")]
    pub target_positions_count: Option<i64>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "traffic_diff")]
    pub traffic_diff: Option<i64>,
    #[serde(default, rename = "traffic_prev")]
    pub traffic_prev: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "url_prev")]
    pub url_prev: Option<String>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "volume_desktop_pct")]
    pub volume_desktop_pct: Option<f64>,
    #[serde(default, rename = "volume_mobile_pct")]
    pub volume_mobile_pct: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OverviewResponse {
    #[serde(default, rename = "overviews")]
    pub overviews: Option<Vec<OverviewsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionsItem {
    #[serde(default, rename = "ahrefs_rank")]
    pub ahrefs_rank: Option<i64>,
    #[serde(default, rename = "backlinks")]
    pub backlinks: Option<i64>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "keywords")]
    pub keywords: Option<i64>,
    #[serde(default, rename = "nr_words")]
    pub nr_words: Option<i64>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<String>,
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "refdomains")]
    pub refdomains: Option<i64>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "top_keyword")]
    pub top_keyword: Option<String>,
    #[serde(default, rename = "top_keyword_volume")]
    pub top_keyword_volume: Option<i64>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "type")]
    pub r#type: Option<Vec<String>>,
    #[serde(default, rename = "update_date")]
    pub update_date: Option<String>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "url_rating")]
    pub url_rating: Option<f64>,
    #[serde(default, rename = "value")]
    pub value: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SerpOverviewResponse {
    #[serde(default, rename = "positions")]
    pub positions: Option<Vec<PositionsItem>>,
}
