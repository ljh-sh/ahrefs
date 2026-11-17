//! Response models for the `site-explorer` API group.

#![allow(missing_docs)]

use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllPlatforms {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Chatgpt {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Copilot {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Gemini {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoogleAiMode {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoogleAiModeKeywords {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoogleAiOverviews {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GoogleAiOverviewsKeywords {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Grok {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Perplexity {
    #[serde(default, rename = "citations")]
    pub citations: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AiResponsesCount {
    #[serde(default, rename = "all_platforms")]
    pub all_platforms: Option<AllPlatforms>,
    #[serde(default, rename = "chatgpt")]
    pub chatgpt: Option<Chatgpt>,
    #[serde(default, rename = "copilot")]
    pub copilot: Option<Copilot>,
    #[serde(default, rename = "gemini")]
    pub gemini: Option<Gemini>,
    #[serde(default, rename = "google_ai_mode")]
    pub google_ai_mode: Option<GoogleAiMode>,
    #[serde(default, rename = "google_ai_mode_keywords")]
    pub google_ai_mode_keywords: Option<GoogleAiModeKeywords>,
    #[serde(default, rename = "google_ai_overviews")]
    pub google_ai_overviews: Option<GoogleAiOverviews>,
    #[serde(default, rename = "google_ai_overviews_keywords")]
    pub google_ai_overviews_keywords: Option<GoogleAiOverviewsKeywords>,
    #[serde(default, rename = "grok")]
    pub grok: Option<Grok>,
    #[serde(default, rename = "perplexity")]
    pub perplexity: Option<Perplexity>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AiResponsesCountResponse {
    #[serde(default, rename = "ai_responses_count")]
    pub ai_responses_count: Option<AiResponsesCount>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BacklinksItem {
    #[serde(default, rename = "ahrefs_rank_source")]
    pub ahrefs_rank_source: Option<i64>,
    #[serde(default, rename = "ahrefs_rank_target")]
    pub ahrefs_rank_target: Option<i64>,
    #[serde(default, rename = "alt")]
    pub alt: Option<String>,
    #[serde(default, rename = "anchor")]
    pub anchor: Option<String>,
    #[serde(default, rename = "broken_redirect_new_target")]
    pub broken_redirect_new_target: Option<String>,
    #[serde(default, rename = "broken_redirect_reason")]
    pub broken_redirect_reason: Option<String>,
    #[serde(default, rename = "broken_redirect_source")]
    pub broken_redirect_source: Option<String>,
    #[serde(default, rename = "class_c")]
    pub class_c: Option<i64>,
    #[serde(default, rename = "discovered_status")]
    pub discovered_status: Option<String>,
    #[serde(default, rename = "domain_rating_source")]
    pub domain_rating_source: Option<f64>,
    #[serde(default, rename = "domain_rating_target")]
    pub domain_rating_target: Option<f64>,
    #[serde(default, rename = "drop_reason")]
    pub drop_reason: Option<String>,
    #[serde(default, rename = "encoding")]
    pub encoding: Option<String>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "first_seen_link")]
    pub first_seen_link: Option<String>,
    #[serde(default, rename = "http_code")]
    pub http_code: Option<i64>,
    #[serde(default, rename = "http_crawl")]
    pub http_crawl: Option<bool>,
    #[serde(default, rename = "ip_source")]
    pub ip_source: Option<String>,
    #[serde(default, rename = "is_alternate")]
    pub is_alternate: Option<bool>,
    #[serde(default, rename = "is_canonical")]
    pub is_canonical: Option<bool>,
    #[serde(default, rename = "is_content")]
    pub is_content: Option<bool>,
    #[serde(default, rename = "is_dofollow")]
    pub is_dofollow: Option<bool>,
    #[serde(default, rename = "is_form")]
    pub is_form: Option<bool>,
    #[serde(default, rename = "is_frame")]
    pub is_frame: Option<bool>,
    #[serde(default, rename = "is_image")]
    pub is_image: Option<bool>,
    #[serde(default, rename = "is_lost")]
    pub is_lost: Option<bool>,
    #[serde(default, rename = "is_new")]
    pub is_new: Option<bool>,
    #[serde(default, rename = "is_nofollow")]
    pub is_nofollow: Option<bool>,
    #[serde(default, rename = "is_redirect")]
    pub is_redirect: Option<bool>,
    #[serde(default, rename = "is_redirect_lost")]
    pub is_redirect_lost: Option<bool>,
    #[serde(default, rename = "is_root_source")]
    pub is_root_source: Option<bool>,
    #[serde(default, rename = "is_root_target")]
    pub is_root_target: Option<bool>,
    #[serde(default, rename = "is_rss")]
    pub is_rss: Option<bool>,
    #[serde(default, rename = "is_spam")]
    pub is_spam: Option<bool>,
    #[serde(default, rename = "is_sponsored")]
    pub is_sponsored: Option<bool>,
    #[serde(default, rename = "is_text")]
    pub is_text: Option<bool>,
    #[serde(default, rename = "is_ugc")]
    pub is_ugc: Option<bool>,
    #[serde(default, rename = "js_crawl")]
    pub js_crawl: Option<bool>,
    #[serde(default, rename = "languages")]
    pub languages: Option<Vec<String>>,
    #[serde(default, rename = "last_seen")]
    pub last_seen: Option<String>,
    #[serde(default, rename = "last_visited")]
    pub last_visited: Option<String>,
    #[serde(default, rename = "link_group_count")]
    pub link_group_count: Option<i64>,
    #[serde(default, rename = "link_type")]
    pub link_type: Option<String>,
    #[serde(default, rename = "linked_domains_source_domain")]
    pub linked_domains_source_domain: Option<i64>,
    #[serde(default, rename = "linked_domains_source_page")]
    pub linked_domains_source_page: Option<i64>,
    #[serde(default, rename = "linked_domains_target_domain")]
    pub linked_domains_target_domain: Option<i64>,
    #[serde(default, rename = "links_external")]
    pub links_external: Option<i64>,
    #[serde(default, rename = "links_internal")]
    pub links_internal: Option<i64>,
    #[serde(default, rename = "lost_reason")]
    pub lost_reason: Option<String>,
    #[serde(default, rename = "name_source")]
    pub name_source: Option<String>,
    #[serde(default, rename = "name_target")]
    pub name_target: Option<String>,
    #[serde(default, rename = "noindex")]
    pub noindex: Option<bool>,
    #[serde(default, rename = "page_category_source")]
    pub page_category_source: Option<String>,
    #[serde(default, rename = "page_size")]
    pub page_size: Option<i64>,
    #[serde(default, rename = "page_type_source")]
    pub page_type_source: Option<String>,
    #[serde(default, rename = "port_source")]
    pub port_source: Option<i64>,
    #[serde(default, rename = "port_target")]
    pub port_target: Option<i64>,
    #[serde(default, rename = "positions")]
    pub positions: Option<i64>,
    #[serde(default, rename = "powered_by")]
    pub powered_by: Option<Vec<String>>,
    #[serde(default, rename = "redirect_code")]
    pub redirect_code: Option<i64>,
    #[serde(default, rename = "redirect_kind")]
    pub redirect_kind: Option<Vec<i64>>,
    #[serde(default, rename = "refdomains_source")]
    pub refdomains_source: Option<i64>,
    #[serde(default, rename = "refdomains_source_domain")]
    pub refdomains_source_domain: Option<i64>,
    #[serde(default, rename = "refdomains_target_domain")]
    pub refdomains_target_domain: Option<i64>,
    #[serde(default, rename = "root_name_source")]
    pub root_name_source: Option<String>,
    #[serde(default, rename = "root_name_target")]
    pub root_name_target: Option<String>,
    #[serde(default, rename = "snippet_left")]
    pub snippet_left: Option<String>,
    #[serde(default, rename = "snippet_right")]
    pub snippet_right: Option<String>,
    #[serde(default, rename = "source_page_author")]
    pub source_page_author: Option<String>,
    #[serde(default, rename = "source_page_publish_date")]
    pub source_page_publish_date: Option<String>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "tld_class_source")]
    pub tld_class_source: Option<String>,
    #[serde(default, rename = "tld_class_target")]
    pub tld_class_target: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "traffic_domain")]
    pub traffic_domain: Option<i64>,
    #[serde(default, rename = "url_from")]
    pub url_from: Option<String>,
    #[serde(default, rename = "url_from_plain")]
    pub url_from_plain: Option<String>,
    #[serde(default, rename = "url_rating_source")]
    pub url_rating_source: Option<f64>,
    #[serde(default, rename = "url_redirect")]
    pub url_redirect: Option<Vec<String>>,
    #[serde(default, rename = "url_redirect_with_target")]
    pub url_redirect_with_target: Option<Vec<String>>,
    #[serde(default, rename = "url_to")]
    pub url_to: Option<String>,
    #[serde(default, rename = "url_to_plain")]
    pub url_to_plain: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AllBacklinksResponse {
    #[serde(default, rename = "backlinks")]
    pub backlinks: Option<Vec<BacklinksItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnchorsItem {
    #[serde(default, rename = "anchor")]
    pub anchor: Option<String>,
    #[serde(default, rename = "dofollow_links")]
    pub dofollow_links: Option<i64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "is_spam")]
    pub is_spam: Option<bool>,
    #[serde(default, rename = "last_seen")]
    pub last_seen: Option<String>,
    #[serde(default, rename = "links_to_target")]
    pub links_to_target: Option<i64>,
    #[serde(default, rename = "lost_links")]
    pub lost_links: Option<i64>,
    #[serde(default, rename = "new_links")]
    pub new_links: Option<i64>,
    #[serde(default, rename = "refdomains")]
    pub refdomains: Option<i64>,
    #[serde(default, rename = "refpages")]
    pub refpages: Option<i64>,
    #[serde(default, rename = "top_domain_rating")]
    pub top_domain_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AnchorsResponse {
    #[serde(default, rename = "anchors")]
    pub anchors: Option<Vec<AnchorsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metrics {
    #[serde(default, rename = "all_time")]
    pub all_time: Option<i64>,
    #[serde(default, rename = "all_time_refdomains")]
    pub all_time_refdomains: Option<i64>,
    #[serde(default, rename = "live")]
    pub live: Option<i64>,
    #[serde(default, rename = "live_refdomains")]
    pub live_refdomains: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BacklinksStatsResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Metrics>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BacklinksItem2 {
    #[serde(default, rename = "ahrefs_rank_source")]
    pub ahrefs_rank_source: Option<i64>,
    #[serde(default, rename = "ahrefs_rank_target")]
    pub ahrefs_rank_target: Option<i64>,
    #[serde(default, rename = "alt")]
    pub alt: Option<String>,
    #[serde(default, rename = "anchor")]
    pub anchor: Option<String>,
    #[serde(default, rename = "class_c")]
    pub class_c: Option<i64>,
    #[serde(default, rename = "domain_rating_source")]
    pub domain_rating_source: Option<f64>,
    #[serde(default, rename = "domain_rating_target")]
    pub domain_rating_target: Option<f64>,
    #[serde(default, rename = "encoding")]
    pub encoding: Option<String>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "first_seen_link")]
    pub first_seen_link: Option<String>,
    #[serde(default, rename = "http_code")]
    pub http_code: Option<i64>,
    #[serde(default, rename = "http_code_target")]
    pub http_code_target: Option<i64>,
    #[serde(default, rename = "http_crawl")]
    pub http_crawl: Option<bool>,
    #[serde(default, rename = "ip_source")]
    pub ip_source: Option<String>,
    #[serde(default, rename = "is_alternate")]
    pub is_alternate: Option<bool>,
    #[serde(default, rename = "is_canonical")]
    pub is_canonical: Option<bool>,
    #[serde(default, rename = "is_content")]
    pub is_content: Option<bool>,
    #[serde(default, rename = "is_dofollow")]
    pub is_dofollow: Option<bool>,
    #[serde(default, rename = "is_form")]
    pub is_form: Option<bool>,
    #[serde(default, rename = "is_frame")]
    pub is_frame: Option<bool>,
    #[serde(default, rename = "is_image")]
    pub is_image: Option<bool>,
    #[serde(default, rename = "is_nofollow")]
    pub is_nofollow: Option<bool>,
    #[serde(default, rename = "is_redirect")]
    pub is_redirect: Option<bool>,
    #[serde(default, rename = "is_root_source")]
    pub is_root_source: Option<bool>,
    #[serde(default, rename = "is_root_target")]
    pub is_root_target: Option<bool>,
    #[serde(default, rename = "is_rss")]
    pub is_rss: Option<bool>,
    #[serde(default, rename = "is_spam")]
    pub is_spam: Option<bool>,
    #[serde(default, rename = "is_sponsored")]
    pub is_sponsored: Option<bool>,
    #[serde(default, rename = "is_text")]
    pub is_text: Option<bool>,
    #[serde(default, rename = "is_ugc")]
    pub is_ugc: Option<bool>,
    #[serde(default, rename = "js_crawl")]
    pub js_crawl: Option<bool>,
    #[serde(default, rename = "languages")]
    pub languages: Option<Vec<String>>,
    #[serde(default, rename = "last_seen")]
    pub last_seen: Option<String>,
    #[serde(default, rename = "last_visited")]
    pub last_visited: Option<String>,
    #[serde(default, rename = "last_visited_target")]
    pub last_visited_target: Option<String>,
    #[serde(default, rename = "link_group_count")]
    pub link_group_count: Option<i64>,
    #[serde(default, rename = "link_type")]
    pub link_type: Option<String>,
    #[serde(default, rename = "linked_domains_source_domain")]
    pub linked_domains_source_domain: Option<i64>,
    #[serde(default, rename = "linked_domains_source_page")]
    pub linked_domains_source_page: Option<i64>,
    #[serde(default, rename = "linked_domains_target_domain")]
    pub linked_domains_target_domain: Option<i64>,
    #[serde(default, rename = "links_external")]
    pub links_external: Option<i64>,
    #[serde(default, rename = "links_internal")]
    pub links_internal: Option<i64>,
    #[serde(default, rename = "name_source")]
    pub name_source: Option<String>,
    #[serde(default, rename = "name_target")]
    pub name_target: Option<String>,
    #[serde(default, rename = "page_category_source")]
    pub page_category_source: Option<String>,
    #[serde(default, rename = "page_size")]
    pub page_size: Option<i64>,
    #[serde(default, rename = "page_type_source")]
    pub page_type_source: Option<String>,
    #[serde(default, rename = "port_source")]
    pub port_source: Option<i64>,
    #[serde(default, rename = "port_target")]
    pub port_target: Option<i64>,
    #[serde(default, rename = "positions")]
    pub positions: Option<i64>,
    #[serde(default, rename = "powered_by")]
    pub powered_by: Option<Vec<String>>,
    #[serde(default, rename = "redirect_code")]
    pub redirect_code: Option<i64>,
    #[serde(default, rename = "redirect_kind")]
    pub redirect_kind: Option<Vec<i64>>,
    #[serde(default, rename = "refdomains_source")]
    pub refdomains_source: Option<i64>,
    #[serde(default, rename = "refdomains_source_domain")]
    pub refdomains_source_domain: Option<i64>,
    #[serde(default, rename = "refdomains_target_domain")]
    pub refdomains_target_domain: Option<i64>,
    #[serde(default, rename = "root_name_source")]
    pub root_name_source: Option<String>,
    #[serde(default, rename = "root_name_target")]
    pub root_name_target: Option<String>,
    #[serde(default, rename = "snippet_left")]
    pub snippet_left: Option<String>,
    #[serde(default, rename = "snippet_right")]
    pub snippet_right: Option<String>,
    #[serde(default, rename = "source_page_author")]
    pub source_page_author: Option<String>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "tld_class_source")]
    pub tld_class_source: Option<String>,
    #[serde(default, rename = "tld_class_target")]
    pub tld_class_target: Option<String>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "traffic_domain")]
    pub traffic_domain: Option<i64>,
    #[serde(default, rename = "url_from")]
    pub url_from: Option<String>,
    #[serde(default, rename = "url_from_plain")]
    pub url_from_plain: Option<String>,
    #[serde(default, rename = "url_rating_source")]
    pub url_rating_source: Option<f64>,
    #[serde(default, rename = "url_redirect")]
    pub url_redirect: Option<Vec<String>>,
    #[serde(default, rename = "url_redirect_with_target")]
    pub url_redirect_with_target: Option<Vec<String>>,
    #[serde(default, rename = "url_to")]
    pub url_to: Option<String>,
    #[serde(default, rename = "url_to_plain")]
    pub url_to_plain: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrokenBacklinksResponse {
    #[serde(default, rename = "backlinks")]
    pub backlinks: Option<Vec<BacklinksItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem {
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "http_code")]
    pub http_code: Option<i64>,
    #[serde(default, rename = "languages")]
    pub languages: Option<Vec<String>>,
    #[serde(default, rename = "last_attempt")]
    pub last_attempt: Option<String>,
    #[serde(default, rename = "last_crawled")]
    pub last_crawled: Option<String>,
    #[serde(default, rename = "page_category")]
    pub page_category: Option<String>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<String>,
    #[serde(default, rename = "publish_date")]
    pub publish_date: Option<String>,
    #[serde(default, rename = "title")]
    pub title: Option<String>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "url_rating")]
    pub url_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CrawledPagesResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRating {
    #[serde(default, rename = "ahrefs_rank")]
    pub ahrefs_rank: Option<i64>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRatingResponse {
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<DomainRating>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRatingsItem {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRatingHistoryResponse {
    #[serde(default, rename = "domain_ratings")]
    pub domain_ratings: Option<Vec<DomainRatingsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "top3")]
    pub top3: Option<i64>,
    #[serde(default, rename = "top4_10")]
    pub top4_10: Option<i64>,
    #[serde(default, rename = "top11_20")]
    pub top11_20: Option<i64>,
    #[serde(default, rename = "top11_plus")]
    pub top11_plus: Option<i64>,
    #[serde(default, rename = "top21_50")]
    pub top21_50: Option<i64>,
    #[serde(default, rename = "top51_plus")]
    pub top51_plus: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsHistoryResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkedanchorsItem {
    #[serde(default, rename = "anchor")]
    pub anchor: Option<String>,
    #[serde(default, rename = "dofollow_links")]
    pub dofollow_links: Option<i64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "linked_domains")]
    pub linked_domains: Option<i64>,
    #[serde(default, rename = "linked_pages")]
    pub linked_pages: Option<i64>,
    #[serde(default, rename = "links_from_target")]
    pub links_from_target: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkedAnchorsExternalResponse {
    #[serde(default, rename = "linkedanchors")]
    pub linkedanchors: Option<Vec<LinkedanchorsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkedanchorsItem2 {
    #[serde(default, rename = "anchor")]
    pub anchor: Option<String>,
    #[serde(default, rename = "dofollow_links")]
    pub dofollow_links: Option<i64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "linked_pages")]
    pub linked_pages: Option<i64>,
    #[serde(default, rename = "links_from_target")]
    pub links_from_target: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkedAnchorsInternalResponse {
    #[serde(default, rename = "linkedanchors")]
    pub linkedanchors: Option<Vec<LinkedanchorsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkeddomainsItem {
    #[serde(default, rename = "dofollow_linked_domains")]
    pub dofollow_linked_domains: Option<i64>,
    #[serde(default, rename = "dofollow_links")]
    pub dofollow_links: Option<i64>,
    #[serde(default, rename = "dofollow_refdomains")]
    pub dofollow_refdomains: Option<i64>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "is_root_domain")]
    pub is_root_domain: Option<bool>,
    #[serde(default, rename = "linked_domain_traffic")]
    pub linked_domain_traffic: Option<i64>,
    #[serde(default, rename = "linked_pages")]
    pub linked_pages: Option<i64>,
    #[serde(default, rename = "links_from_target")]
    pub links_from_target: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkeddomainsResponse {
    #[serde(default, rename = "linkeddomains")]
    pub linkeddomains: Option<Vec<LinkeddomainsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metrics2 {
    #[serde(default, rename = "org_cost")]
    pub org_cost: Option<i64>,
    #[serde(default, rename = "org_keywords")]
    pub org_keywords: Option<i64>,
    #[serde(default, rename = "org_keywords_1_3")]
    pub org_keywords_1_3: Option<i64>,
    #[serde(default, rename = "org_traffic")]
    pub org_traffic: Option<i64>,
    #[serde(default, rename = "paid_cost")]
    pub paid_cost: Option<i64>,
    #[serde(default, rename = "paid_keywords")]
    pub paid_keywords: Option<i64>,
    #[serde(default, rename = "paid_pages")]
    pub paid_pages: Option<i64>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Metrics2>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "org_cost")]
    pub org_cost: Option<i64>,
    #[serde(default, rename = "org_keywords")]
    pub org_keywords: Option<i64>,
    #[serde(default, rename = "org_keywords_1_3")]
    pub org_keywords_1_3: Option<i64>,
    #[serde(default, rename = "org_traffic")]
    pub org_traffic: Option<i64>,
    #[serde(default, rename = "paid_cost")]
    pub paid_cost: Option<i64>,
    #[serde(default, rename = "paid_keywords")]
    pub paid_keywords: Option<i64>,
    #[serde(default, rename = "paid_pages")]
    pub paid_pages: Option<i64>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsByCountryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem2 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "org_cost")]
    pub org_cost: Option<i64>,
    #[serde(default, rename = "org_traffic")]
    pub org_traffic: Option<i64>,
    #[serde(default, rename = "paid_cost")]
    pub paid_cost: Option<i64>,
    #[serde(default, rename = "paid_traffic")]
    pub paid_traffic: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsItem {
    #[serde(default, rename = "competitor_domain")]
    pub competitor_domain: Option<String>,
    #[serde(default, rename = "competitor_url")]
    pub competitor_url: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "group_mode")]
    pub group_mode: Option<String>,
    #[serde(default, rename = "keywords_common")]
    pub keywords_common: Option<i64>,
    #[serde(default, rename = "keywords_competitor")]
    pub keywords_competitor: Option<i64>,
    #[serde(default, rename = "keywords_target")]
    pub keywords_target: Option<i64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
    #[serde(default, rename = "pages_diff")]
    pub pages_diff: Option<i64>,
    #[serde(default, rename = "pages_merged")]
    pub pages_merged: Option<i64>,
    #[serde(default, rename = "pages_prev")]
    pub pages_prev: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<i64>,
    #[serde(default, rename = "traffic_diff")]
    pub traffic_diff: Option<i64>,
    #[serde(default, rename = "traffic_merged")]
    pub traffic_merged: Option<i64>,
    #[serde(default, rename = "traffic_prev")]
    pub traffic_prev: Option<i64>,
    #[serde(default, rename = "value")]
    pub value: Option<i64>,
    #[serde(default, rename = "value_diff")]
    pub value_diff: Option<i64>,
    #[serde(default, rename = "value_merged")]
    pub value_merged: Option<i64>,
    #[serde(default, rename = "value_prev")]
    pub value_prev: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrganicCompetitorsResponse {
    #[serde(default, rename = "competitors")]
    pub competitors: Option<Vec<CompetitorsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem2 {
    #[serde(default, rename = "all_positions")]
    pub all_positions: Option<Vec<Value>>,
    #[serde(default, rename = "all_positions_prev")]
    pub all_positions_prev: Option<Vec<Value>>,
    #[serde(default, rename = "best_position")]
    pub best_position: Option<i64>,
    #[serde(default, rename = "best_position_diff")]
    pub best_position_diff: Option<i64>,
    #[serde(default, rename = "best_position_has_thumbnail")]
    pub best_position_has_thumbnail: Option<bool>,
    #[serde(default, rename = "best_position_has_thumbnail_prev")]
    pub best_position_has_thumbnail_prev: Option<bool>,
    #[serde(default, rename = "best_position_has_video")]
    pub best_position_has_video: Option<bool>,
    #[serde(default, rename = "best_position_has_video_prev")]
    pub best_position_has_video_prev: Option<bool>,
    #[serde(default, rename = "best_position_kind")]
    pub best_position_kind: Option<String>,
    #[serde(default, rename = "best_position_kind_merged")]
    pub best_position_kind_merged: Option<String>,
    #[serde(default, rename = "best_position_kind_prev")]
    pub best_position_kind_prev: Option<String>,
    #[serde(default, rename = "best_position_prev")]
    pub best_position_prev: Option<i64>,
    #[serde(default, rename = "best_position_set")]
    pub best_position_set: Option<String>,
    #[serde(default, rename = "best_position_set_prev")]
    pub best_position_set_prev: Option<String>,
    #[serde(default, rename = "best_position_url")]
    pub best_position_url: Option<String>,
    #[serde(default, rename = "best_position_url_prev")]
    pub best_position_url_prev: Option<String>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "cpc_merged")]
    pub cpc_merged: Option<i64>,
    #[serde(default, rename = "cpc_prev")]
    pub cpc_prev: Option<i64>,
    #[serde(default, rename = "entities")]
    pub entities: Option<Vec<Value>>,
    #[serde(default, rename = "is_best_position_set_top_3")]
    pub is_best_position_set_top_3: Option<bool>,
    #[serde(default, rename = "is_best_position_set_top_3_prev")]
    pub is_best_position_set_top_3_prev: Option<bool>,
    #[serde(default, rename = "is_best_position_set_top_4_10")]
    pub is_best_position_set_top_4_10: Option<bool>,
    #[serde(default, rename = "is_best_position_set_top_4_10_prev")]
    pub is_best_position_set_top_4_10_prev: Option<bool>,
    #[serde(default, rename = "is_best_position_set_top_11_50")]
    pub is_best_position_set_top_11_50: Option<bool>,
    #[serde(default, rename = "is_best_position_set_top_11_50_prev")]
    pub is_best_position_set_top_11_50_prev: Option<bool>,
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
    #[serde(default, rename = "keyword_country")]
    pub keyword_country: Option<String>,
    #[serde(default, rename = "keyword_difficulty")]
    pub keyword_difficulty: Option<i64>,
    #[serde(default, rename = "keyword_difficulty_merged")]
    pub keyword_difficulty_merged: Option<i64>,
    #[serde(default, rename = "keyword_difficulty_prev")]
    pub keyword_difficulty_prev: Option<i64>,
    #[serde(default, rename = "keyword_language")]
    pub keyword_language: Option<Vec<String>>,
    #[serde(default, rename = "keyword_list_id")]
    pub keyword_list_id: Option<Vec<String>>,
    #[serde(default, rename = "keyword_list_name")]
    pub keyword_list_name: Option<Vec<String>>,
    #[serde(default, rename = "keyword_merged")]
    pub keyword_merged: Option<String>,
    #[serde(default, rename = "keyword_prev")]
    pub keyword_prev: Option<String>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "language_prev")]
    pub language_prev: Option<String>,
    #[serde(default, rename = "last_update")]
    pub last_update: Option<String>,
    #[serde(default, rename = "last_update_prev")]
    pub last_update_prev: Option<String>,
    #[serde(default, rename = "serp_features")]
    pub serp_features: Option<Vec<String>>,
    #[serde(default, rename = "serp_features_count")]
    pub serp_features_count: Option<i64>,
    #[serde(default, rename = "serp_features_count_prev")]
    pub serp_features_count_prev: Option<i64>,
    #[serde(default, rename = "serp_features_merged")]
    pub serp_features_merged: Option<Vec<String>>,
    #[serde(default, rename = "serp_features_prev")]
    pub serp_features_prev: Option<Vec<String>>,
    #[serde(default, rename = "serp_target_main_positions_count")]
    pub serp_target_main_positions_count: Option<i64>,
    #[serde(default, rename = "serp_target_main_positions_count_prev")]
    pub serp_target_main_positions_count_prev: Option<i64>,
    #[serde(default, rename = "serp_target_positions_count")]
    pub serp_target_positions_count: Option<i64>,
    #[serde(default, rename = "serp_target_positions_count_prev")]
    pub serp_target_positions_count_prev: Option<i64>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "sum_paid_traffic")]
    pub sum_paid_traffic: Option<i64>,
    #[serde(default, rename = "sum_paid_traffic_merged")]
    pub sum_paid_traffic_merged: Option<i64>,
    #[serde(default, rename = "sum_paid_traffic_prev")]
    pub sum_paid_traffic_prev: Option<i64>,
    #[serde(default, rename = "sum_traffic")]
    pub sum_traffic: Option<i64>,
    #[serde(default, rename = "sum_traffic_merged")]
    pub sum_traffic_merged: Option<i64>,
    #[serde(default, rename = "sum_traffic_prev")]
    pub sum_traffic_prev: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
    #[serde(default, rename = "volume_desktop_pct")]
    pub volume_desktop_pct: Option<f64>,
    #[serde(default, rename = "volume_merged")]
    pub volume_merged: Option<i64>,
    #[serde(default, rename = "volume_mobile_pct")]
    pub volume_mobile_pct: Option<f64>,
    #[serde(default, rename = "volume_prev")]
    pub volume_prev: Option<i64>,
    #[serde(default, rename = "words")]
    pub words: Option<i64>,
    #[serde(default, rename = "words_merged")]
    pub words_merged: Option<i64>,
    #[serde(default, rename = "words_prev")]
    pub words_prev: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrganicKeywordsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metrics3 {
    #[serde(default, rename = "linked_domains")]
    pub linked_domains: Option<i64>,
    #[serde(default, rename = "linked_domains_dofollow")]
    pub linked_domains_dofollow: Option<i64>,
    #[serde(default, rename = "outgoing_links")]
    pub outgoing_links: Option<i64>,
    #[serde(default, rename = "outgoing_links_dofollow")]
    pub outgoing_links_dofollow: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OutlinksStatsResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Metrics3>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem2 {
    #[serde(default, rename = "dofollow_to_target")]
    pub dofollow_to_target: Option<i64>,
    #[serde(default, rename = "first_seen_link")]
    pub first_seen_link: Option<String>,
    #[serde(default, rename = "http_code_target")]
    pub http_code_target: Option<i64>,
    #[serde(default, rename = "is_spam")]
    pub is_spam: Option<bool>,
    #[serde(default, rename = "languages_target")]
    pub languages_target: Option<Vec<String>>,
    #[serde(default, rename = "last_seen")]
    pub last_seen: Option<String>,
    #[serde(default, rename = "last_visited_source")]
    pub last_visited_source: Option<String>,
    #[serde(default, rename = "last_visited_target")]
    pub last_visited_target: Option<String>,
    #[serde(default, rename = "links_to_target")]
    pub links_to_target: Option<i64>,
    #[serde(default, rename = "lost_links_to_target")]
    pub lost_links_to_target: Option<i64>,
    #[serde(default, rename = "new_links_to_target")]
    pub new_links_to_target: Option<i64>,
    #[serde(default, rename = "nofollow_to_target")]
    pub nofollow_to_target: Option<i64>,
    #[serde(default, rename = "page_type_target")]
    pub page_type_target: Option<String>,
    #[serde(default, rename = "powered_by_target")]
    pub powered_by_target: Option<Vec<String>>,
    #[serde(default, rename = "redirects_to_target")]
    pub redirects_to_target: Option<i64>,
    #[serde(default, rename = "refdomains_target")]
    pub refdomains_target: Option<i64>,
    #[serde(default, rename = "target_redirect")]
    pub target_redirect: Option<String>,
    #[serde(default, rename = "title_target")]
    pub title_target: Option<String>,
    #[serde(default, rename = "top_domain_rating_source")]
    pub top_domain_rating_source: Option<f64>,
    #[serde(default, rename = "url_rating_target")]
    pub url_rating_target: Option<f64>,
    #[serde(default, rename = "url_to")]
    pub url_to: Option<String>,
    #[serde(default, rename = "url_to_plain")]
    pub url_to_plain: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesByBacklinksResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem3 {
    #[serde(default, rename = "canonical_to_target")]
    pub canonical_to_target: Option<i64>,
    #[serde(default, rename = "dofollow_to_target")]
    pub dofollow_to_target: Option<i64>,
    #[serde(default, rename = "first_seen_link")]
    pub first_seen_link: Option<String>,
    #[serde(default, rename = "http_code_target")]
    pub http_code_target: Option<i64>,
    #[serde(default, rename = "languages_target")]
    pub languages_target: Option<Vec<String>>,
    #[serde(default, rename = "last_seen")]
    pub last_seen: Option<String>,
    #[serde(default, rename = "last_visited_source")]
    pub last_visited_source: Option<String>,
    #[serde(default, rename = "last_visited_target")]
    pub last_visited_target: Option<String>,
    #[serde(default, rename = "links_to_target")]
    pub links_to_target: Option<i64>,
    #[serde(default, rename = "nofollow_to_target")]
    pub nofollow_to_target: Option<i64>,
    #[serde(default, rename = "page_type_target")]
    pub page_type_target: Option<String>,
    #[serde(default, rename = "powered_by_target")]
    pub powered_by_target: Option<Vec<String>>,
    #[serde(default, rename = "redirects_to_target")]
    pub redirects_to_target: Option<i64>,
    #[serde(default, rename = "target_redirect")]
    pub target_redirect: Option<String>,
    #[serde(default, rename = "title_target")]
    pub title_target: Option<String>,
    #[serde(default, rename = "url_rating_target")]
    pub url_rating_target: Option<f64>,
    #[serde(default, rename = "url_to")]
    pub url_to: Option<String>,
    #[serde(default, rename = "url_to_plain")]
    pub url_to_plain: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesByInternalLinksResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pages {
    #[serde(default, rename = "range0_pages")]
    pub range0_pages: Option<i64>,
    #[serde(default, rename = "range1k_pages")]
    pub range1k_pages: Option<i64>,
    #[serde(default, rename = "range1k_traffic")]
    pub range1k_traffic: Option<i64>,
    #[serde(default, rename = "range5k_pages")]
    pub range5k_pages: Option<i64>,
    #[serde(default, rename = "range5k_traffic")]
    pub range5k_traffic: Option<i64>,
    #[serde(default, rename = "range10k_pages")]
    pub range10k_pages: Option<i64>,
    #[serde(default, rename = "range10k_plus_pages")]
    pub range10k_plus_pages: Option<i64>,
    #[serde(default, rename = "range10k_plus_traffic")]
    pub range10k_plus_traffic: Option<i64>,
    #[serde(default, rename = "range10k_traffic")]
    pub range10k_traffic: Option<i64>,
    #[serde(default, rename = "range100_pages")]
    pub range100_pages: Option<i64>,
    #[serde(default, rename = "range100_traffic")]
    pub range100_traffic: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesByTrafficResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Pages>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem4 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesHistoryResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem4>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem5 {
    #[serde(default, rename = "ads_count")]
    pub ads_count: Option<i64>,
    #[serde(default, rename = "ads_count_diff")]
    pub ads_count_diff: Option<i64>,
    #[serde(default, rename = "ads_count_prev")]
    pub ads_count_prev: Option<i64>,
    #[serde(default, rename = "keywords")]
    pub keywords: Option<i64>,
    #[serde(default, rename = "keywords_diff")]
    pub keywords_diff: Option<i64>,
    #[serde(default, rename = "keywords_diff_percent")]
    pub keywords_diff_percent: Option<i64>,
    #[serde(default, rename = "keywords_merged")]
    pub keywords_merged: Option<i64>,
    #[serde(default, rename = "keywords_prev")]
    pub keywords_prev: Option<i64>,
    #[serde(default, rename = "raw_url")]
    pub raw_url: Option<String>,
    #[serde(default, rename = "raw_url_prev")]
    pub raw_url_prev: Option<String>,
    #[serde(default, rename = "referring_domains")]
    pub referring_domains: Option<i64>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "sum_traffic")]
    pub sum_traffic: Option<i64>,
    #[serde(default, rename = "sum_traffic_merged")]
    pub sum_traffic_merged: Option<i64>,
    #[serde(default, rename = "sum_traffic_prev")]
    pub sum_traffic_prev: Option<i64>,
    #[serde(default, rename = "top_keyword")]
    pub top_keyword: Option<String>,
    #[serde(default, rename = "top_keyword_best_position")]
    pub top_keyword_best_position: Option<i64>,
    #[serde(default, rename = "top_keyword_best_position_diff")]
    pub top_keyword_best_position_diff: Option<i64>,
    #[serde(default, rename = "top_keyword_best_position_kind")]
    pub top_keyword_best_position_kind: Option<String>,
    #[serde(default, rename = "top_keyword_best_position_kind_prev")]
    pub top_keyword_best_position_kind_prev: Option<String>,
    #[serde(default, rename = "top_keyword_best_position_prev")]
    pub top_keyword_best_position_prev: Option<i64>,
    #[serde(default, rename = "top_keyword_best_position_title")]
    pub top_keyword_best_position_title: Option<String>,
    #[serde(default, rename = "top_keyword_best_position_title_prev")]
    pub top_keyword_best_position_title_prev: Option<String>,
    #[serde(default, rename = "top_keyword_country")]
    pub top_keyword_country: Option<String>,
    #[serde(default, rename = "top_keyword_country_prev")]
    pub top_keyword_country_prev: Option<String>,
    #[serde(default, rename = "top_keyword_prev")]
    pub top_keyword_prev: Option<String>,
    #[serde(default, rename = "top_keyword_volume")]
    pub top_keyword_volume: Option<i64>,
    #[serde(default, rename = "top_keyword_volume_prev")]
    pub top_keyword_volume_prev: Option<i64>,
    #[serde(default, rename = "traffic_diff")]
    pub traffic_diff: Option<i64>,
    #[serde(default, rename = "traffic_diff_percent")]
    pub traffic_diff_percent: Option<i64>,
    #[serde(default, rename = "ur")]
    pub ur: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "url_prev")]
    pub url_prev: Option<String>,
    #[serde(default, rename = "value")]
    pub value: Option<i64>,
    #[serde(default, rename = "value_diff")]
    pub value_diff: Option<i64>,
    #[serde(default, rename = "value_diff_percent")]
    pub value_diff_percent: Option<i64>,
    #[serde(default, rename = "value_merged")]
    pub value_merged: Option<i64>,
    #[serde(default, rename = "value_prev")]
    pub value_prev: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaidPagesResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem5>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefdomainsItem {
    #[serde(default, rename = "dofollow_linked_domains")]
    pub dofollow_linked_domains: Option<i64>,
    #[serde(default, rename = "dofollow_links")]
    pub dofollow_links: Option<i64>,
    #[serde(default, rename = "dofollow_refdomains")]
    pub dofollow_refdomains: Option<i64>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "first_seen")]
    pub first_seen: Option<String>,
    #[serde(default, rename = "ip_source")]
    pub ip_source: Option<String>,
    #[serde(default, rename = "is_root_domain")]
    pub is_root_domain: Option<bool>,
    #[serde(default, rename = "is_spam")]
    pub is_spam: Option<bool>,
    #[serde(default, rename = "last_seen")]
    pub last_seen: Option<String>,
    #[serde(default, rename = "links_to_target")]
    pub links_to_target: Option<i64>,
    #[serde(default, rename = "lost_links")]
    pub lost_links: Option<i64>,
    #[serde(default, rename = "new_links")]
    pub new_links: Option<i64>,
    #[serde(default, rename = "positions_source_domain")]
    pub positions_source_domain: Option<i64>,
    #[serde(default, rename = "traffic_domain")]
    pub traffic_domain: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefdomainsResponse {
    #[serde(default, rename = "refdomains")]
    pub refdomains: Option<Vec<RefdomainsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefdomainsItem2 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "refdomains")]
    pub refdomains: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RefdomainsHistoryResponse {
    #[serde(default, rename = "refdomains")]
    pub refdomains: Option<Vec<RefdomainsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem6 {
    #[serde(default, rename = "ai_responses_chatgpt")]
    pub ai_responses_chatgpt: Option<i64>,
    #[serde(default, rename = "ai_responses_copilot")]
    pub ai_responses_copilot: Option<i64>,
    #[serde(default, rename = "ai_responses_gemini")]
    pub ai_responses_gemini: Option<i64>,
    #[serde(default, rename = "ai_responses_google_ai_mode")]
    pub ai_responses_google_ai_mode: Option<i64>,
    #[serde(default, rename = "ai_responses_google_ai_overviews")]
    pub ai_responses_google_ai_overviews: Option<i64>,
    #[serde(default, rename = "ai_responses_grok")]
    pub ai_responses_grok: Option<i64>,
    #[serde(default, rename = "ai_responses_perplexity")]
    pub ai_responses_perplexity: Option<i64>,
    #[serde(default, rename = "keywords")]
    pub keywords: Option<i64>,
    #[serde(default, rename = "keywords_diff")]
    pub keywords_diff: Option<i64>,
    #[serde(default, rename = "keywords_diff_percent")]
    pub keywords_diff_percent: Option<i64>,
    #[serde(default, rename = "keywords_merged")]
    pub keywords_merged: Option<i64>,
    #[serde(default, rename = "keywords_prev")]
    pub keywords_prev: Option<i64>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<String>,
    #[serde(default, rename = "raw_url")]
    pub raw_url: Option<String>,
    #[serde(default, rename = "raw_url_prev")]
    pub raw_url_prev: Option<String>,
    #[serde(default, rename = "referring_domains")]
    pub referring_domains: Option<i64>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "sum_traffic")]
    pub sum_traffic: Option<i64>,
    #[serde(default, rename = "sum_traffic_merged")]
    pub sum_traffic_merged: Option<i64>,
    #[serde(default, rename = "sum_traffic_prev")]
    pub sum_traffic_prev: Option<i64>,
    #[serde(default, rename = "top_keyword")]
    pub top_keyword: Option<String>,
    #[serde(default, rename = "top_keyword_best_position")]
    pub top_keyword_best_position: Option<i64>,
    #[serde(default, rename = "top_keyword_best_position_diff")]
    pub top_keyword_best_position_diff: Option<i64>,
    #[serde(default, rename = "top_keyword_best_position_kind")]
    pub top_keyword_best_position_kind: Option<String>,
    #[serde(default, rename = "top_keyword_best_position_kind_prev")]
    pub top_keyword_best_position_kind_prev: Option<String>,
    #[serde(default, rename = "top_keyword_best_position_prev")]
    pub top_keyword_best_position_prev: Option<i64>,
    #[serde(default, rename = "top_keyword_best_position_title")]
    pub top_keyword_best_position_title: Option<String>,
    #[serde(default, rename = "top_keyword_best_position_title_prev")]
    pub top_keyword_best_position_title_prev: Option<String>,
    #[serde(default, rename = "top_keyword_country")]
    pub top_keyword_country: Option<String>,
    #[serde(default, rename = "top_keyword_country_prev")]
    pub top_keyword_country_prev: Option<String>,
    #[serde(default, rename = "top_keyword_prev")]
    pub top_keyword_prev: Option<String>,
    #[serde(default, rename = "top_keyword_volume")]
    pub top_keyword_volume: Option<i64>,
    #[serde(default, rename = "top_keyword_volume_prev")]
    pub top_keyword_volume_prev: Option<i64>,
    #[serde(default, rename = "traffic_diff")]
    pub traffic_diff: Option<i64>,
    #[serde(default, rename = "traffic_diff_percent")]
    pub traffic_diff_percent: Option<i64>,
    #[serde(default, rename = "ur")]
    pub ur: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "url_prev")]
    pub url_prev: Option<String>,
    #[serde(default, rename = "value")]
    pub value: Option<i64>,
    #[serde(default, rename = "value_diff")]
    pub value_diff: Option<i64>,
    #[serde(default, rename = "value_diff_percent")]
    pub value_diff_percent: Option<i64>,
    #[serde(default, rename = "value_merged")]
    pub value_merged: Option<i64>,
    #[serde(default, rename = "value_prev")]
    pub value_prev: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopPagesResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem6>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem3 {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "total_search_volume")]
    pub total_search_volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TotalSearchVolumeHistoryResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficMixItem {
    #[serde(default, rename = "channel")]
    pub channel: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CountriesItem {
    #[serde(default, rename = "place")]
    pub place: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubregionsItem {
    #[serde(default, rename = "place")]
    pub place: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContinentsItem {
    #[serde(default, rename = "place")]
    pub place: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VisitorsFrom {
    #[serde(default, rename = "countries")]
    pub countries: Option<Vec<CountriesItem>>,
    #[serde(default, rename = "subregions")]
    pub subregions: Option<Vec<SubregionsItem>>,
    #[serde(default, rename = "continents")]
    pub continents: Option<Vec<ContinentsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WebsitesItem {
    #[serde(default, rename = "affinity")]
    pub affinity: Option<f64>,
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "dr")]
    pub dr: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "search_share")]
    pub search_share: Option<f64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "shared_visitors")]
    pub shared_visitors: Option<i64>,
    #[serde(default, rename = "traffic_mix")]
    pub traffic_mix: Option<Vec<TrafficMixItem>>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visitors_from")]
    pub visitors_from: Option<VisitorsFrom>,
    #[serde(default, rename = "website")]
    pub website: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficAudienceOverlapResponse {
    #[serde(default, rename = "websites")]
    pub websites: Option<Vec<WebsitesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelsItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "channel")]
    pub channel: Option<String>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficChannelsResponse {
    #[serde(default, rename = "channels")]
    pub channels: Option<Vec<ChannelsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "partial")]
    pub partial: Option<bool>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficMixItem2 {
    #[serde(default, rename = "channel")]
    pub channel: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CountriesItem2 {
    #[serde(default, rename = "place")]
    pub place: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubregionsItem2 {
    #[serde(default, rename = "place")]
    pub place: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContinentsItem2 {
    #[serde(default, rename = "place")]
    pub place: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct VisitorsFrom2 {
    #[serde(default, rename = "countries")]
    pub countries: Option<Vec<CountriesItem2>>,
    #[serde(default, rename = "subregions")]
    pub subregions: Option<Vec<SubregionsItem2>>,
    #[serde(default, rename = "continents")]
    pub continents: Option<Vec<ContinentsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WebsitesItem2 {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "dr")]
    pub dr: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "shared_of_target")]
    pub shared_of_target: Option<f64>,
    #[serde(default, rename = "shared_of_website")]
    pub shared_of_website: Option<f64>,
    #[serde(default, rename = "shared_visitors")]
    pub shared_visitors: Option<i64>,
    #[serde(default, rename = "traffic_mix")]
    pub traffic_mix: Option<Vec<TrafficMixItem2>>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visitors_from")]
    pub visitors_from: Option<VisitorsFrom2>,
    #[serde(default, rename = "website")]
    pub website: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficCompetitorOverlapResponse {
    #[serde(default, rename = "websites")]
    pub websites: Option<Vec<WebsitesItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LocationsItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficGeographyResponse {
    #[serde(default, rename = "locations")]
    pub locations: Option<Vec<LocationsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LandingPagesItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "page_pageviews")]
    pub page_pageviews: Option<i64>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<String>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "ur")]
    pub ur: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficLandingPagesResponse {
    #[serde(default, rename = "landing_pages")]
    pub landing_pages: Option<Vec<LandingPagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Metrics4 {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "date_from")]
    pub date_from: Option<String>,
    #[serde(default, rename = "date_to")]
    pub date_to: Option<String>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficMetricsResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Metrics4>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupsItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "group")]
    pub group: Option<String>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficPageClassificationsResponse {
    #[serde(default, rename = "groups")]
    pub groups: Option<Vec<GroupsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreviousPagesItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<String>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "ur")]
    pub ur: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficPreviousPagesResponse {
    #[serde(default, rename = "previous_pages")]
    pub previous_pages: Option<Vec<PreviousPagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PreviousWebsitesItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "pages")]
    pub pages: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "previous_website")]
    pub previous_website: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficPreviousWebsitesResponse {
    #[serde(default, rename = "previous_websites")]
    pub previous_websites: Option<Vec<PreviousWebsitesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferringPagesItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<String>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "ur")]
    pub ur: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficReferringPagesResponse {
    #[serde(default, rename = "referring_pages")]
    pub referring_pages: Option<Vec<ReferringPagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferringSubdomainsItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "dr")]
    pub dr: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "referring_subdomain")]
    pub referring_subdomain: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficReferringSubdomainsResponse {
    #[serde(default, rename = "referring_subdomains")]
    pub referring_subdomains: Option<Vec<ReferringSubdomainsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferringWebsitesItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "dr")]
    pub dr: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "referring_website")]
    pub referring_website: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficReferringWebsitesResponse {
    #[serde(default, rename = "referring_websites")]
    pub referring_websites: Option<Vec<ReferringWebsitesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SearchTermsItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "cpc")]
    pub cpc: Option<i64>,
    #[serde(default, rename = "kd")]
    pub kd: Option<i64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "position")]
    pub position: Option<i64>,
    #[serde(default, rename = "position_url")]
    pub position_url: Option<String>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "term")]
    pub term: Option<String>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
    #[serde(default, rename = "volume")]
    pub volume: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficSearchTermsResponse {
    #[serde(default, rename = "search_terms")]
    pub search_terms: Option<Vec<SearchTermsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourcesItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "dr")]
    pub dr: Option<f64>,
    #[serde(default, rename = "main_channel")]
    pub main_channel: Option<String>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficSourcesResponse {
    #[serde(default, rename = "sources")]
    pub sources: Option<Vec<SourcesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem7 {
    #[serde(default, rename = "avg_time_on_page")]
    pub avg_time_on_page: Option<i64>,
    #[serde(default, rename = "entry_bounce_rate")]
    pub entry_bounce_rate: Option<f64>,
    #[serde(default, rename = "entry_pages_per_visit")]
    pub entry_pages_per_visit: Option<f64>,
    #[serde(default, rename = "entry_pageviews")]
    pub entry_pageviews: Option<i64>,
    #[serde(default, rename = "entry_visit_duration")]
    pub entry_visit_duration: Option<i64>,
    #[serde(default, rename = "exit_rate")]
    pub exit_rate: Option<f64>,
    #[serde(default, rename = "organic_traffic")]
    pub organic_traffic: Option<i64>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<String>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "ur")]
    pub ur: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficTopPagesResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem7>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UtmParamsItem {
    #[serde(default, rename = "bounce_rate")]
    pub bounce_rate: Option<f64>,
    #[serde(default, rename = "pages_per_visit")]
    pub pages_per_visit: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "share")]
    pub share: Option<f64>,
    #[serde(default, rename = "utm_param")]
    pub utm_param: Option<String>,
    #[serde(default, rename = "visit_duration")]
    pub visit_duration: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrafficUtmParamsResponse {
    #[serde(default, rename = "utm_params")]
    pub utm_params: Option<Vec<UtmParamsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UrlRatingsItem {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "url_rating")]
    pub url_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UrlRatingHistoryResponse {
    #[serde(default, rename = "url_ratings")]
    pub url_ratings: Option<Vec<UrlRatingsItem>>,
}
