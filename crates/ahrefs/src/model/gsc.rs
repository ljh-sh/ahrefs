//! Response models for the `gsc` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem {
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "keyword_country")]
    pub keyword_country: Option<String>,
    #[serde(default, rename = "keyword_difficulty")]
    pub keyword_difficulty: Option<i64>,
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnonymousQueriesResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem {
    #[serde(default, rename = "average_ctr_percent")]
    pub average_ctr_percent: Option<f64>,
    #[serde(default, rename = "keyword_count")]
    pub keyword_count: Option<i64>,
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CtrByPositionResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem2 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "ctr")]
    pub ctr: Option<f64>,
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<i64>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "position")]
    pub position: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem2 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "ctr")]
    pub ctr: Option<f64>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<i64>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "keyword_list_labels")]
    pub keyword_list_labels: Option<Vec<String>>,
    #[serde(default, rename = "position")]
    pub position: Option<f64>,
    #[serde(default, rename = "top_url")]
    pub top_url: Option<String>,
    #[serde(default, rename = "urls_count")]
    pub urls_count: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem3 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsByCountryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem4 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "ctr")]
    pub ctr: Option<f64>,
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<i64>,
    #[serde(default, rename = "page")]
    pub page: Option<String>,
    #[serde(default, rename = "position")]
    pub position: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PageHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem4>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "ctr")]
    pub ctr: Option<f64>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<i64>,
    #[serde(default, rename = "keywords_count")]
    pub keywords_count: Option<i64>,
    #[serde(default, rename = "page")]
    pub page: Option<String>,
    #[serde(default, rename = "position")]
    pub position: Option<f64>,
    #[serde(default, rename = "top_keyword")]
    pub top_keyword: Option<String>,
    #[serde(default, rename = "traffic_value")]
    pub traffic_value: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem5 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "total_pages")]
    pub total_pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem5>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem6 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<f64>,
    #[serde(default, rename = "ctr")]
    pub ctr: Option<f64>,
    #[serde(default, rename = "device")]
    pub device: Option<String>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<f64>,
    #[serde(default, rename = "position")]
    pub position: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PerformanceByDeviceResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem6>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem7 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "clicks_percent")]
    pub clicks_percent: Option<f64>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<i64>,
    #[serde(default, rename = "impressions_percent")]
    pub impressions_percent: Option<f64>,
    #[serde(default, rename = "keyword_count")]
    pub keyword_count: Option<i64>,
    #[serde(default, rename = "keyword_count_percent")]
    pub keyword_count_percent: Option<f64>,
    #[serde(default, rename = "position_range")]
    pub position_range: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PerformanceByPositionResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem7>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem8 {
    #[serde(default, rename = "clicks")]
    pub clicks: Option<i64>,
    #[serde(default, rename = "ctr")]
    pub ctr: Option<f64>,
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "impressions")]
    pub impressions: Option<i64>,
    #[serde(default, rename = "position")]
    pub position: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PerformanceHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem8>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem9 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "position_1_to_3")]
    pub position_1_to_3: Option<i64>,
    #[serde(default, rename = "position_4_to_10")]
    pub position_4_to_10: Option<i64>,
    #[serde(default, rename = "position_11_to_20")]
    pub position_11_to_20: Option<i64>,
    #[serde(default, rename = "position_21_to_50")]
    pub position_21_to_50: Option<i64>,
    #[serde(default, rename = "position_51_plus")]
    pub position_51_plus: Option<i64>,
    #[serde(default, rename = "total")]
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionsHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem9>>,
}
