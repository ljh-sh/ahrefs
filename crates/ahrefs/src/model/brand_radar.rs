//! Response models for the `brand-radar` API group.

#![allow(missing_docs)]

use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandSentimentItem {
    #[serde(default, rename = "brand")]
    pub brand: Option<String>,
    #[serde(default, rename = "sentiment")]
    pub sentiment: Option<String>,
    #[serde(default, rename = "aspect")]
    pub aspect: Option<String>,
    #[serde(default, rename = "snippet")]
    pub snippet: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinksItem {
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AiResponsesItem {
    #[serde(default, rename = "brand_sentiment")]
    pub brand_sentiment: Option<Vec<BrandSentimentItem>>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "data_source")]
    pub data_source: Option<String>,
    #[serde(default, rename = "funnel_stage")]
    pub funnel_stage: Option<String>,
    #[serde(default, rename = "intent")]
    pub intent: Option<Vec<String>>,
    #[serde(default, rename = "last_updated")]
    pub last_updated: Option<String>,
    #[serde(default, rename = "links")]
    pub links: Option<Vec<LinksItem>>,
    #[serde(default, rename = "question")]
    pub question: Option<String>,
    #[serde(default, rename = "response")]
    pub response: Option<String>,
    #[serde(default, rename = "search_queries")]
    pub search_queries: Option<Vec<String>>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AiResponsesResponse {
    #[serde(default, rename = "ai_responses")]
    pub ai_responses: Option<Vec<AiResponsesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "date")]
    pub date: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CitationsHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem2 {
    #[serde(default, rename = "brand")]
    pub brand: Option<String>,
    #[serde(default, rename = "entity_type")]
    pub entity_type: Option<String>,
    #[serde(default, rename = "no_tracked_brands")]
    pub no_tracked_brands: Option<i64>,
    #[serde(default, rename = "only_competitors_brands")]
    pub only_competitors_brands: Option<i64>,
    #[serde(default, rename = "only_target_brand")]
    pub only_target_brand: Option<i64>,
    #[serde(default, rename = "target_and_competitors_brands")]
    pub target_and_competitors_brands: Option<i64>,
    #[serde(default, rename = "total")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CitationsOverviewResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainsItem {
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "mentions")]
    pub mentions: Option<Vec<Value>>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
    #[serde(default, rename = "responses")]
    pub responses: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CitedDomainsResponse {
    #[serde(default, rename = "domains")]
    pub domains: Option<Vec<DomainsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem {
    #[serde(default, rename = "mentions")]
    pub mentions: Option<Vec<Value>>,
    #[serde(default, rename = "responses")]
    pub responses: Option<i64>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CitedPagesResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem3 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImpressionsHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem4 {
    #[serde(default, rename = "brand")]
    pub brand: Option<String>,
    #[serde(default, rename = "entity_type")]
    pub entity_type: Option<String>,
    #[serde(default, rename = "no_tracked_brands")]
    pub no_tracked_brands: Option<i64>,
    #[serde(default, rename = "only_competitors_brands")]
    pub only_competitors_brands: Option<i64>,
    #[serde(default, rename = "only_target_brand")]
    pub only_target_brand: Option<i64>,
    #[serde(default, rename = "target_and_competitors_brands")]
    pub target_and_competitors_brands: Option<i64>,
    #[serde(default, rename = "total")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ImpressionsOverviewResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem4>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem5 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "mentions")]
    pub mentions: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MentionsHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem5>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem6 {
    #[serde(default, rename = "brand")]
    pub brand: Option<String>,
    #[serde(default, rename = "entity_type")]
    pub entity_type: Option<String>,
    #[serde(default, rename = "no_tracked_brands")]
    pub no_tracked_brands: Option<i64>,
    #[serde(default, rename = "only_competitors_brands")]
    pub only_competitors_brands: Option<i64>,
    #[serde(default, rename = "only_target_brand")]
    pub only_target_brand: Option<i64>,
    #[serde(default, rename = "target_and_competitors_brands")]
    pub target_and_competitors_brands: Option<i64>,
    #[serde(default, rename = "total")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MentionsOverviewResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem6>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShareOfVoiceItem {
    #[serde(default, rename = "brand")]
    pub brand: Option<String>,
    #[serde(default, rename = "entity_type")]
    pub entity_type: Option<String>,
    #[serde(default, rename = "share_of_voice")]
    pub share_of_voice: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem7 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "share_of_voice")]
    pub share_of_voice: Option<Vec<ShareOfVoiceItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SovHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem7>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem8 {
    #[serde(default, rename = "brand")]
    pub brand: Option<String>,
    #[serde(default, rename = "entity_type")]
    pub entity_type: Option<String>,
    #[serde(default, rename = "share_of_voice")]
    pub share_of_voice: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SovOverviewResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem8>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MentionsItem {
    #[serde(default, rename = "entity")]
    pub entity: Option<String>,
    #[serde(default, rename = "responses")]
    pub responses: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopicsItem {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "mentions")]
    pub mentions: Option<Vec<MentionsItem>>,
    #[serde(default, rename = "responses")]
    pub responses: Option<i64>,
    #[serde(default, rename = "topic")]
    pub topic: Option<String>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopicsResponse {
    #[serde(default, rename = "topics")]
    pub topics: Option<Vec<TopicsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem2 {
    #[serde(default, rename = "content")]
    pub content: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<i64>,
    #[serde(default, rename = "referring_domains")]
    pub referring_domains: Option<i64>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<f64>,
    #[serde(default, rename = "traffic_value")]
    pub traffic_value: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WebPagesResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem2>>,
}
