//! Response models for the `serp-overview` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PositionsItem {
    #[serde(default, rename = "ahrefs_rank")]
    pub ahrefs_rank: Option<i64>,
    #[serde(default, rename = "backlinks")]
    pub backlinks: Option<i64>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "keywords")]
    pub keywords: Option<i64>,
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
