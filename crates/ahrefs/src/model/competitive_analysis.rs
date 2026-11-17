//! Response models for the `competitive-analysis` API group.

#![allow(missing_docs)]

use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Target {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
    #[serde(default, rename = "position_has_video")]
    pub position_has_video: Option<bool>,
    #[serde(default, rename = "position_has_thumbnail")]
    pub position_has_thumbnail: Option<bool>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor1 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor2 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor3 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor4 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor5 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor6 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor7 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor8 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor9 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor10 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor11 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor12 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor13 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor14 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor15 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor16 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor17 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor18 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor19 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Competitor20 {
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "position_kind")]
    pub position_kind: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "paid_position")]
    pub paid_position: Option<i64>,
    #[serde(default, rename = "paid_url")]
    pub paid_url: Option<String>,
    #[serde(default, rename = "paid_position_kind")]
    pub paid_position_kind: Option<String>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
    #[serde(default, rename = "paid_title")]
    pub paid_title: Option<String>,
    #[serde(default, rename = "paid_display_url")]
    pub paid_display_url: Option<String>,
    #[serde(default, rename = "paid_description")]
    pub paid_description: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem {
    #[serde(default, rename = "competitors_count")]
    pub competitors_count: Option<i64>,
    #[serde(default, rename = "competitors_count_in_range")]
    pub competitors_count_in_range: Option<i64>,
    #[serde(default, rename = "competitors_paid_bottom_count")]
    pub competitors_paid_bottom_count: Option<i64>,
    #[serde(default, rename = "competitors_paid_count")]
    pub competitors_paid_count: Option<i64>,
    #[serde(default, rename = "competitors_paid_top_count")]
    pub competitors_paid_top_count: Option<i64>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "entities")]
    pub entities: Option<Vec<Value>>,
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "keyword_difficulty")]
    pub keyword_difficulty: Option<i64>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "last_update")]
    pub last_update: Option<String>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_features_count")]
    pub serp_features_count: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "intents")]
    pub intents: Option<Vec<String>>,
    #[serde(default, rename = "target")]
    pub target: Option<Target>,
    #[serde(default, rename = "competitor_1")]
    pub competitor_1: Option<Competitor1>,
    #[serde(default, rename = "competitor_2")]
    pub competitor_2: Option<Competitor2>,
    #[serde(default, rename = "competitor_3")]
    pub competitor_3: Option<Competitor3>,
    #[serde(default, rename = "competitor_4")]
    pub competitor_4: Option<Competitor4>,
    #[serde(default, rename = "competitor_5")]
    pub competitor_5: Option<Competitor5>,
    #[serde(default, rename = "competitor_6")]
    pub competitor_6: Option<Competitor6>,
    #[serde(default, rename = "competitor_7")]
    pub competitor_7: Option<Competitor7>,
    #[serde(default, rename = "competitor_8")]
    pub competitor_8: Option<Competitor8>,
    #[serde(default, rename = "competitor_9")]
    pub competitor_9: Option<Competitor9>,
    #[serde(default, rename = "competitor_10")]
    pub competitor_10: Option<Competitor10>,
    #[serde(default, rename = "competitor_11")]
    pub competitor_11: Option<Competitor11>,
    #[serde(default, rename = "competitor_12")]
    pub competitor_12: Option<Competitor12>,
    #[serde(default, rename = "competitor_13")]
    pub competitor_13: Option<Competitor13>,
    #[serde(default, rename = "competitor_14")]
    pub competitor_14: Option<Competitor14>,
    #[serde(default, rename = "competitor_15")]
    pub competitor_15: Option<Competitor15>,
    #[serde(default, rename = "competitor_16")]
    pub competitor_16: Option<Competitor16>,
    #[serde(default, rename = "competitor_17")]
    pub competitor_17: Option<Competitor17>,
    #[serde(default, rename = "competitor_18")]
    pub competitor_18: Option<Competitor18>,
    #[serde(default, rename = "competitor_19")]
    pub competitor_19: Option<Competitor19>,
    #[serde(default, rename = "competitor_20")]
    pub competitor_20: Option<Competitor20>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentGapResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferringDomainsItem {
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "intersect")]
    pub intersect: Option<i64>,
    #[serde(default, rename = "links_to_competitor_1")]
    pub links_to_competitor_1: Option<i64>,
    #[serde(default, rename = "links_to_competitor_2")]
    pub links_to_competitor_2: Option<i64>,
    #[serde(default, rename = "links_to_competitor_3")]
    pub links_to_competitor_3: Option<i64>,
    #[serde(default, rename = "links_to_competitor_4")]
    pub links_to_competitor_4: Option<i64>,
    #[serde(default, rename = "links_to_competitor_5")]
    pub links_to_competitor_5: Option<i64>,
    #[serde(default, rename = "links_to_competitor_6")]
    pub links_to_competitor_6: Option<i64>,
    #[serde(default, rename = "links_to_competitor_7")]
    pub links_to_competitor_7: Option<i64>,
    #[serde(default, rename = "links_to_competitor_8")]
    pub links_to_competitor_8: Option<i64>,
    #[serde(default, rename = "links_to_competitor_9")]
    pub links_to_competitor_9: Option<i64>,
    #[serde(default, rename = "links_to_competitor_10")]
    pub links_to_competitor_10: Option<i64>,
    #[serde(default, rename = "links_to_competitor_11")]
    pub links_to_competitor_11: Option<i64>,
    #[serde(default, rename = "links_to_competitor_12")]
    pub links_to_competitor_12: Option<i64>,
    #[serde(default, rename = "links_to_competitor_13")]
    pub links_to_competitor_13: Option<i64>,
    #[serde(default, rename = "links_to_competitor_14")]
    pub links_to_competitor_14: Option<i64>,
    #[serde(default, rename = "links_to_competitor_15")]
    pub links_to_competitor_15: Option<i64>,
    #[serde(default, rename = "links_to_competitor_16")]
    pub links_to_competitor_16: Option<i64>,
    #[serde(default, rename = "links_to_competitor_17")]
    pub links_to_competitor_17: Option<i64>,
    #[serde(default, rename = "links_to_competitor_18")]
    pub links_to_competitor_18: Option<i64>,
    #[serde(default, rename = "links_to_competitor_19")]
    pub links_to_competitor_19: Option<i64>,
    #[serde(default, rename = "links_to_competitor_20")]
    pub links_to_competitor_20: Option<i64>,
    #[serde(default, rename = "links_to_target")]
    pub links_to_target: Option<i64>,
    #[serde(default, rename = "traffic_domain")]
    pub traffic_domain: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkIntersectRefdomainsResponse {
    #[serde(default, rename = "referring-domains")]
    pub referring_domains: Option<Vec<ReferringDomainsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferringPagesItem {
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "intersect")]
    pub intersect: Option<i64>,
    #[serde(default, rename = "links_to_competitor_1")]
    pub links_to_competitor_1: Option<i64>,
    #[serde(default, rename = "links_to_competitor_2")]
    pub links_to_competitor_2: Option<i64>,
    #[serde(default, rename = "links_to_competitor_3")]
    pub links_to_competitor_3: Option<i64>,
    #[serde(default, rename = "links_to_competitor_4")]
    pub links_to_competitor_4: Option<i64>,
    #[serde(default, rename = "links_to_competitor_5")]
    pub links_to_competitor_5: Option<i64>,
    #[serde(default, rename = "links_to_competitor_6")]
    pub links_to_competitor_6: Option<i64>,
    #[serde(default, rename = "links_to_competitor_7")]
    pub links_to_competitor_7: Option<i64>,
    #[serde(default, rename = "links_to_competitor_8")]
    pub links_to_competitor_8: Option<i64>,
    #[serde(default, rename = "links_to_competitor_9")]
    pub links_to_competitor_9: Option<i64>,
    #[serde(default, rename = "links_to_competitor_10")]
    pub links_to_competitor_10: Option<i64>,
    #[serde(default, rename = "links_to_competitor_11")]
    pub links_to_competitor_11: Option<i64>,
    #[serde(default, rename = "links_to_competitor_12")]
    pub links_to_competitor_12: Option<i64>,
    #[serde(default, rename = "links_to_competitor_13")]
    pub links_to_competitor_13: Option<i64>,
    #[serde(default, rename = "links_to_competitor_14")]
    pub links_to_competitor_14: Option<i64>,
    #[serde(default, rename = "links_to_competitor_15")]
    pub links_to_competitor_15: Option<i64>,
    #[serde(default, rename = "links_to_competitor_16")]
    pub links_to_competitor_16: Option<i64>,
    #[serde(default, rename = "links_to_competitor_17")]
    pub links_to_competitor_17: Option<i64>,
    #[serde(default, rename = "links_to_competitor_18")]
    pub links_to_competitor_18: Option<i64>,
    #[serde(default, rename = "links_to_competitor_19")]
    pub links_to_competitor_19: Option<i64>,
    #[serde(default, rename = "links_to_competitor_20")]
    pub links_to_competitor_20: Option<i64>,
    #[serde(default, rename = "links_to_target")]
    pub links_to_target: Option<i64>,
    #[serde(default, rename = "refdomains")]
    pub refdomains: Option<i64>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "traffic_domain")]
    pub traffic_domain: Option<i64>,
    #[serde(default, rename = "traffic_page")]
    pub traffic_page: Option<i64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "url_rating")]
    pub url_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkIntersectRefpagesResponse {
    #[serde(default, rename = "referring-pages")]
    pub referring_pages: Option<Vec<ReferringPagesItem>>,
}
