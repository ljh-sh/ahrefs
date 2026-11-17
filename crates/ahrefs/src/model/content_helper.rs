//! Response models for the `content-helper` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem {
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "score")]
    pub score: Option<i64>,
    #[serde(default, rename = "extracts")]
    pub extracts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "score")]
    pub score: Option<i64>,
    #[serde(default, rename = "extracts")]
    pub extracts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopicsItem {
    #[serde(default, rename = "avg_score")]
    pub avg_score: Option<f64>,
    #[serde(default, rename = "description")]
    pub description: Option<String>,
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem>>,
    #[serde(default, rename = "pages_covering")]
    pub pages_covering: Option<i64>,
    #[serde(default, rename = "serp_date")]
    pub serp_date: Option<String>,
    #[serde(default, rename = "target")]
    pub target: Option<Target>,
    #[serde(default, rename = "topic")]
    pub topic: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopicsResponse {
    #[serde(default, rename = "topics")]
    pub topics: Option<Vec<TopicsItem>>,
}
