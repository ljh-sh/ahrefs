//! Response models for the `batch-analysis` API group.

#![allow(missing_docs)]

use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TargetsItem {
    #[serde(default, rename = "ahrefs_rank")]
    pub ahrefs_rank: Option<i64>,
    #[serde(default, rename = "backlinks")]
    pub backlinks: Option<i64>,
    #[serde(default, rename = "backlinks_dofollow")]
    pub backlinks_dofollow: Option<i64>,
    #[serde(default, rename = "backlinks_internal")]
    pub backlinks_internal: Option<i64>,
    #[serde(default, rename = "backlinks_nofollow")]
    pub backlinks_nofollow: Option<i64>,
    #[serde(default, rename = "backlinks_redirect")]
    pub backlinks_redirect: Option<i64>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "index")]
    pub index: Option<i64>,
    #[serde(default, rename = "ip")]
    pub ip: Option<String>,
    #[serde(default, rename = "linked_domains")]
    pub linked_domains: Option<i64>,
    #[serde(default, rename = "linked_domains_dofollow")]
    pub linked_domains_dofollow: Option<i64>,
    #[serde(default, rename = "mode")]
    pub mode: Option<String>,
    #[serde(default, rename = "org_cost")]
    pub org_cost: Option<i64>,
    #[serde(default, rename = "org_keywords")]
    pub org_keywords: Option<i64>,
    #[serde(default, rename = "org_keywords_1_3")]
    pub org_keywords_1_3: Option<i64>,
    #[serde(default, rename = "org_keywords_4_10")]
    pub org_keywords_4_10: Option<i64>,
    #[serde(default, rename = "org_keywords_11_20")]
    pub org_keywords_11_20: Option<i64>,
    #[serde(default, rename = "org_keywords_21_50")]
    pub org_keywords_21_50: Option<i64>,
    #[serde(default, rename = "org_keywords_51_plus")]
    pub org_keywords_51_plus: Option<i64>,
    #[serde(default, rename = "org_traffic")]
    pub org_traffic: Option<i64>,
    #[serde(default, rename = "org_traffic_top_by_country")]
    pub org_traffic_top_by_country: Option<Vec<Vec<Value>>>,
    #[serde(default, rename = "outgoing_links")]
    pub outgoing_links: Option<i64>,
    #[serde(default, rename = "outgoing_links_dofollow")]
    pub outgoing_links_dofollow: Option<i64>,
    #[serde(default, rename = "paid_ads")]
    pub paid_ads: Option<i64>,
    #[serde(default, rename = "paid_cost")]
    pub paid_cost: Option<i64>,
    #[serde(default, rename = "paid_keywords")]
    pub paid_keywords: Option<i64>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "protocol")]
    pub protocol: Option<String>,
    #[serde(default, rename = "refdomains")]
    pub refdomains: Option<i64>,
    #[serde(default, rename = "refdomains_dofollow")]
    pub refdomains_dofollow: Option<i64>,
    #[serde(default, rename = "refdomains_nofollow")]
    pub refdomains_nofollow: Option<i64>,
    #[serde(default, rename = "refips")]
    pub refips: Option<i64>,
    #[serde(default, rename = "refips_subnets")]
    pub refips_subnets: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "url_rating")]
    pub url_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchAnalysisResponse {
    #[serde(default, rename = "targets")]
    pub targets: Option<Vec<TargetsItem>>,
}
