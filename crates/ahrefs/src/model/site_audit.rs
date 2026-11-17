//! Response models for the `site-audit` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssuesItem {
    #[serde(default, rename = "added")]
    pub added: Option<i64>,
    #[serde(default, rename = "category")]
    pub category: Option<String>,
    #[serde(default, rename = "change")]
    pub change: Option<i64>,
    #[serde(default, rename = "crawled")]
    pub crawled: Option<i64>,
    #[serde(default, rename = "importance")]
    pub importance: Option<String>,
    #[serde(default, rename = "is_indexable")]
    pub is_indexable: Option<bool>,
    #[serde(default, rename = "issue_id")]
    pub issue_id: Option<String>,
    #[serde(default, rename = "missing")]
    pub missing: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "new")]
    pub new: Option<i64>,
    #[serde(default, rename = "removed")]
    pub removed: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IssuesResponse {
    #[serde(default, rename = "issues")]
    pub issues: Option<Vec<IssuesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PageContent {
    #[serde(default, rename = "crawl_datetime")]
    pub crawl_datetime: Option<String>,
    #[serde(default, rename = "page_text")]
    pub page_text: Option<String>,
    #[serde(default, rename = "page_text_md")]
    pub page_text_md: Option<String>,
    #[serde(default, rename = "raw_html")]
    pub raw_html: Option<String>,
    #[serde(default, rename = "rendered_html")]
    pub rendered_html: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PageContentResponse {
    #[serde(default, rename = "page-content")]
    pub page_content: Option<PageContent>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PagesItem {
    #[serde(default, rename = "ai_content_level")]
    pub ai_content_level: Option<String>,
    #[serde(default, rename = "ai_content_level_prev")]
    pub ai_content_level_prev: Option<String>,
    #[serde(default, rename = "ai_content_status")]
    pub ai_content_status: Option<String>,
    #[serde(default, rename = "ai_content_status_prev")]
    pub ai_content_status_prev: Option<String>,
    #[serde(default, rename = "allowed_search_bots")]
    pub allowed_search_bots: Option<Vec<String>>,
    #[serde(default, rename = "allowed_search_bots_prev")]
    pub allowed_search_bots_prev: Option<Vec<String>>,
    #[serde(default, rename = "allowed_training_bots")]
    pub allowed_training_bots: Option<Vec<String>>,
    #[serde(default, rename = "allowed_training_bots_prev")]
    pub allowed_training_bots_prev: Option<Vec<String>>,
    #[serde(default, rename = "alternate")]
    pub alternate: Option<i64>,
    #[serde(default, rename = "alternate_diff")]
    pub alternate_diff: Option<i64>,
    #[serde(default, rename = "alternate_prev")]
    pub alternate_prev: Option<i64>,
    #[serde(default, rename = "backlinks")]
    pub backlinks: Option<i64>,
    #[serde(default, rename = "backlinks_diff")]
    pub backlinks_diff: Option<i64>,
    #[serde(default, rename = "backlinks_prev")]
    pub backlinks_prev: Option<i64>,
    #[serde(default, rename = "blocked_search_bots")]
    pub blocked_search_bots: Option<Vec<String>>,
    #[serde(default, rename = "blocked_search_bots_prev")]
    pub blocked_search_bots_prev: Option<Vec<String>>,
    #[serde(default, rename = "blocked_training_bots")]
    pub blocked_training_bots: Option<Vec<String>>,
    #[serde(default, rename = "blocked_training_bots_prev")]
    pub blocked_training_bots_prev: Option<Vec<String>>,
    #[serde(default, rename = "canonical")]
    pub canonical: Option<String>,
    #[serde(default, rename = "canonical_code")]
    pub canonical_code: Option<i64>,
    #[serde(default, rename = "canonical_code_prev")]
    pub canonical_code_prev: Option<i64>,
    #[serde(default, rename = "canonical_counts")]
    pub canonical_counts: Option<i64>,
    #[serde(default, rename = "canonical_counts_diff")]
    pub canonical_counts_diff: Option<i64>,
    #[serde(default, rename = "canonical_counts_prev")]
    pub canonical_counts_prev: Option<i64>,
    #[serde(default, rename = "canonical_group_hash")]
    pub canonical_group_hash: Option<i64>,
    #[serde(default, rename = "canonical_group_hash_prev")]
    pub canonical_group_hash_prev: Option<i64>,
    #[serde(default, rename = "canonical_is_canonical")]
    pub canonical_is_canonical: Option<bool>,
    #[serde(default, rename = "canonical_is_canonical_prev")]
    pub canonical_is_canonical_prev: Option<bool>,
    #[serde(default, rename = "canonical_no_crawl_reason")]
    pub canonical_no_crawl_reason: Option<String>,
    #[serde(default, rename = "canonical_no_crawl_reason_prev")]
    pub canonical_no_crawl_reason_prev: Option<String>,
    #[serde(default, rename = "canonical_prev")]
    pub canonical_prev: Option<String>,
    #[serde(default, rename = "canonical_scheme")]
    pub canonical_scheme: Option<String>,
    #[serde(default, rename = "canonical_scheme_prev")]
    pub canonical_scheme_prev: Option<String>,
    #[serde(default, rename = "compliant")]
    pub compliant: Option<bool>,
    #[serde(default, rename = "compliant_prev")]
    pub compliant_prev: Option<bool>,
    #[serde(default, rename = "compression")]
    pub compression: Option<String>,
    #[serde(default, rename = "compression_prev")]
    pub compression_prev: Option<String>,
    #[serde(default, rename = "content_available_without_js")]
    pub content_available_without_js: Option<i64>,
    #[serde(default, rename = "content_available_without_js_diff")]
    pub content_available_without_js_diff: Option<i64>,
    #[serde(default, rename = "content_available_without_js_prev")]
    pub content_available_without_js_prev: Option<i64>,
    #[serde(default, rename = "content_encoding")]
    pub content_encoding: Option<String>,
    #[serde(default, rename = "content_encoding_prev")]
    pub content_encoding_prev: Option<String>,
    #[serde(default, rename = "content_length")]
    pub content_length: Option<i64>,
    #[serde(default, rename = "content_length_diff")]
    pub content_length_diff: Option<i64>,
    #[serde(default, rename = "content_length_prev")]
    pub content_length_prev: Option<i64>,
    #[serde(default, rename = "content_nr_word")]
    pub content_nr_word: Option<i64>,
    #[serde(default, rename = "content_nr_word_diff")]
    pub content_nr_word_diff: Option<i64>,
    #[serde(default, rename = "content_nr_word_prev")]
    pub content_nr_word_prev: Option<i64>,
    #[serde(default, rename = "content_type")]
    pub content_type: Option<String>,
    #[serde(default, rename = "content_type_prev")]
    pub content_type_prev: Option<String>,
    #[serde(default, rename = "css_no_crawl_reason")]
    pub css_no_crawl_reason: Option<Vec<String>>,
    #[serde(default, rename = "css_no_crawl_reason_prev")]
    pub css_no_crawl_reason_prev: Option<Vec<String>>,
    #[serde(default, rename = "curl_code")]
    pub curl_code: Option<i64>,
    #[serde(default, rename = "curl_code_prev")]
    pub curl_code_prev: Option<i64>,
    #[serde(default, rename = "depth")]
    pub depth: Option<i64>,
    #[serde(default, rename = "depth_diff")]
    pub depth_diff: Option<i64>,
    #[serde(default, rename = "depth_prev")]
    pub depth_prev: Option<i64>,
    #[serde(default, rename = "dofollow")]
    pub dofollow: Option<i64>,
    #[serde(default, rename = "dofollow_diff")]
    pub dofollow_diff: Option<i64>,
    #[serde(default, rename = "dofollow_prev")]
    pub dofollow_prev: Option<i64>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "duplicate_content")]
    pub duplicate_content: Option<i64>,
    #[serde(default, rename = "duplicate_content_canonical_hreflang")]
    pub duplicate_content_canonical_hreflang: Option<i64>,
    #[serde(default, rename = "duplicate_content_canonical_hreflang_diff")]
    pub duplicate_content_canonical_hreflang_diff: Option<i64>,
    #[serde(default, rename = "duplicate_content_canonical_hreflang_prev")]
    pub duplicate_content_canonical_hreflang_prev: Option<i64>,
    #[serde(default, rename = "duplicate_content_diff")]
    pub duplicate_content_diff: Option<i64>,
    #[serde(default, rename = "duplicate_content_prev")]
    pub duplicate_content_prev: Option<i64>,
    #[serde(default, rename = "duplicate_description")]
    pub duplicate_description: Option<i64>,
    #[serde(default, rename = "duplicate_description_canonical_hreflang")]
    pub duplicate_description_canonical_hreflang: Option<i64>,
    #[serde(default, rename = "duplicate_description_canonical_hreflang_diff")]
    pub duplicate_description_canonical_hreflang_diff: Option<i64>,
    #[serde(default, rename = "duplicate_description_canonical_hreflang_prev")]
    pub duplicate_description_canonical_hreflang_prev: Option<i64>,
    #[serde(default, rename = "duplicate_description_diff")]
    pub duplicate_description_diff: Option<i64>,
    #[serde(default, rename = "duplicate_description_prev")]
    pub duplicate_description_prev: Option<i64>,
    #[serde(default, rename = "duplicate_group_identifier")]
    pub duplicate_group_identifier: Option<i64>,
    #[serde(default, rename = "duplicate_group_identifier_prev")]
    pub duplicate_group_identifier_prev: Option<i64>,
    #[serde(default, rename = "duplicate_h1")]
    pub duplicate_h1: Option<i64>,
    #[serde(default, rename = "duplicate_h1_canonical_hreflang")]
    pub duplicate_h1_canonical_hreflang: Option<i64>,
    #[serde(default, rename = "duplicate_h1_canonical_hreflang_diff")]
    pub duplicate_h1_canonical_hreflang_diff: Option<i64>,
    #[serde(default, rename = "duplicate_h1_canonical_hreflang_prev")]
    pub duplicate_h1_canonical_hreflang_prev: Option<i64>,
    #[serde(default, rename = "duplicate_h1_diff")]
    pub duplicate_h1_diff: Option<i64>,
    #[serde(default, rename = "duplicate_h1_prev")]
    pub duplicate_h1_prev: Option<i64>,
    #[serde(default, rename = "duplicate_title")]
    pub duplicate_title: Option<i64>,
    #[serde(default, rename = "duplicate_title_canonical_hreflang")]
    pub duplicate_title_canonical_hreflang: Option<i64>,
    #[serde(default, rename = "duplicate_title_canonical_hreflang_diff")]
    pub duplicate_title_canonical_hreflang_diff: Option<i64>,
    #[serde(default, rename = "duplicate_title_canonical_hreflang_prev")]
    pub duplicate_title_canonical_hreflang_prev: Option<i64>,
    #[serde(default, rename = "duplicate_title_diff")]
    pub duplicate_title_diff: Option<i64>,
    #[serde(default, rename = "duplicate_title_prev")]
    pub duplicate_title_prev: Option<i64>,
    #[serde(default, rename = "edu")]
    pub edu: Option<i64>,
    #[serde(default, rename = "edu_diff")]
    pub edu_diff: Option<i64>,
    #[serde(default, rename = "edu_prev")]
    pub edu_prev: Option<i64>,
    #[serde(default, rename = "external_code")]
    pub external_code: Option<Vec<i64>>,
    #[serde(default, rename = "external_code_prev")]
    pub external_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "external_link_anchor")]
    pub external_link_anchor: Option<Vec<String>>,
    #[serde(default, rename = "external_link_anchor_prev")]
    pub external_link_anchor_prev: Option<Vec<String>>,
    #[serde(default, rename = "external_link_domain")]
    pub external_link_domain: Option<Vec<String>>,
    #[serde(default, rename = "external_link_domain_prev")]
    pub external_link_domain_prev: Option<Vec<String>>,
    #[serde(default, rename = "external_links")]
    pub external_links: Option<Vec<String>>,
    #[serde(default, rename = "external_links_is_canonical")]
    pub external_links_is_canonical: Option<Vec<bool>>,
    #[serde(default, rename = "external_links_is_canonical_prev")]
    pub external_links_is_canonical_prev: Option<Vec<bool>>,
    #[serde(default, rename = "external_links_prev")]
    pub external_links_prev: Option<Vec<String>>,
    #[serde(default, rename = "external_no_crawl_reason")]
    pub external_no_crawl_reason: Option<Vec<String>>,
    #[serde(default, rename = "external_no_crawl_reason_prev")]
    pub external_no_crawl_reason_prev: Option<Vec<String>>,
    #[serde(default, rename = "external_scheme")]
    pub external_scheme: Option<Vec<String>>,
    #[serde(default, rename = "external_scheme_prev")]
    pub external_scheme_prev: Option<Vec<String>>,
    #[serde(default, rename = "final_redirect")]
    pub final_redirect: Option<String>,
    #[serde(default, rename = "final_redirect_code")]
    pub final_redirect_code: Option<i64>,
    #[serde(default, rename = "final_redirect_code_prev")]
    pub final_redirect_code_prev: Option<i64>,
    #[serde(default, rename = "final_redirect_no_crawl_reason")]
    pub final_redirect_no_crawl_reason: Option<String>,
    #[serde(default, rename = "final_redirect_no_crawl_reason_prev")]
    pub final_redirect_no_crawl_reason_prev: Option<String>,
    #[serde(default, rename = "final_redirect_prev")]
    pub final_redirect_prev: Option<String>,
    #[serde(default, rename = "found_in_sitemaps")]
    pub found_in_sitemaps: Option<Vec<String>>,
    #[serde(default, rename = "found_in_sitemaps_length")]
    pub found_in_sitemaps_length: Option<i64>,
    #[serde(default, rename = "found_in_sitemaps_length_diff")]
    pub found_in_sitemaps_length_diff: Option<i64>,
    #[serde(default, rename = "found_in_sitemaps_length_prev")]
    pub found_in_sitemaps_length_prev: Option<i64>,
    #[serde(default, rename = "found_in_sitemaps_prev")]
    pub found_in_sitemaps_prev: Option<Vec<String>>,
    #[serde(default, rename = "gov")]
    pub gov: Option<i64>,
    #[serde(default, rename = "gov_diff")]
    pub gov_diff: Option<i64>,
    #[serde(default, rename = "gov_prev")]
    pub gov_prev: Option<i64>,
    #[serde(default, rename = "h1")]
    pub h1: Option<Vec<String>>,
    #[serde(default, rename = "h1_length")]
    pub h1_length: Option<Vec<i64>>,
    #[serde(default, rename = "h1_length_prev")]
    pub h1_length_prev: Option<Vec<i64>>,
    #[serde(default, rename = "h1_prev")]
    pub h1_prev: Option<Vec<String>>,
    #[serde(default, rename = "h2")]
    pub h2: Option<Vec<String>>,
    #[serde(default, rename = "h2_prev")]
    pub h2_prev: Option<Vec<String>>,
    #[serde(default, rename = "hash_content")]
    pub hash_content: Option<i64>,
    #[serde(default, rename = "hash_content_prev")]
    pub hash_content_prev: Option<i64>,
    #[serde(default, rename = "hash_descriptions")]
    pub hash_descriptions: Option<Vec<i64>>,
    #[serde(default, rename = "hash_descriptions_prev")]
    pub hash_descriptions_prev: Option<Vec<i64>>,
    #[serde(default, rename = "hash_h1")]
    pub hash_h1: Option<Vec<i64>>,
    #[serde(default, rename = "hash_h1_prev")]
    pub hash_h1_prev: Option<Vec<i64>>,
    #[serde(default, rename = "hash_text")]
    pub hash_text: Option<i64>,
    #[serde(default, rename = "hash_text_prev")]
    pub hash_text_prev: Option<i64>,
    #[serde(default, rename = "hash_titles")]
    pub hash_titles: Option<Vec<i64>>,
    #[serde(default, rename = "hash_titles_prev")]
    pub hash_titles_prev: Option<Vec<i64>>,
    #[serde(default, rename = "hreflang")]
    pub hreflang: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_code_is_valid")]
    pub hreflang_code_is_valid: Option<Vec<bool>>,
    #[serde(default, rename = "hreflang_code_is_valid_prev")]
    pub hreflang_code_is_valid_prev: Option<Vec<bool>>,
    #[serde(default, rename = "hreflang_country")]
    pub hreflang_country: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_country_prev")]
    pub hreflang_country_prev: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_group_hash")]
    pub hreflang_group_hash: Option<i64>,
    #[serde(default, rename = "hreflang_group_hash_prev")]
    pub hreflang_group_hash_prev: Option<i64>,
    #[serde(default, rename = "hreflang_issues")]
    pub hreflang_issues: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_issues_prev")]
    pub hreflang_issues_prev: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_language")]
    pub hreflang_language: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_language_prev")]
    pub hreflang_language_prev: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_link")]
    pub hreflang_link: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_link_is_canonical")]
    pub hreflang_link_is_canonical: Option<Vec<bool>>,
    #[serde(default, rename = "hreflang_link_is_canonical_prev")]
    pub hreflang_link_is_canonical_prev: Option<Vec<bool>>,
    #[serde(default, rename = "hreflang_link_prev")]
    pub hreflang_link_prev: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_no_crawl_reason")]
    pub hreflang_no_crawl_reason: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_no_crawl_reason_prev")]
    pub hreflang_no_crawl_reason_prev: Option<Vec<String>>,
    #[serde(default, rename = "hreflang_prev")]
    pub hreflang_prev: Option<Vec<String>>,
    #[serde(default, rename = "html_lang")]
    pub html_lang: Option<String>,
    #[serde(default, rename = "html_lang_code_is_valid")]
    pub html_lang_code_is_valid: Option<bool>,
    #[serde(default, rename = "html_lang_code_is_valid_prev")]
    pub html_lang_code_is_valid_prev: Option<bool>,
    #[serde(default, rename = "html_lang_country")]
    pub html_lang_country: Option<String>,
    #[serde(default, rename = "html_lang_country_prev")]
    pub html_lang_country_prev: Option<String>,
    #[serde(default, rename = "html_lang_language")]
    pub html_lang_language: Option<String>,
    #[serde(default, rename = "html_lang_language_prev")]
    pub html_lang_language_prev: Option<String>,
    #[serde(default, rename = "html_lang_prev")]
    pub html_lang_prev: Option<String>,
    #[serde(default, rename = "http_code")]
    pub http_code: Option<i64>,
    #[serde(default, rename = "http_code_prev")]
    pub http_code_prev: Option<i64>,
    #[serde(default, rename = "http_header")]
    pub http_header: Option<Vec<String>>,
    #[serde(default, rename = "http_header_prev")]
    pub http_header_prev: Option<Vec<String>>,
    #[serde(default, rename = "http_header_robots")]
    pub http_header_robots: Option<Vec<String>>,
    #[serde(default, rename = "http_header_robots_prev")]
    pub http_header_robots_prev: Option<Vec<String>>,
    #[serde(default, rename = "http_headers_size")]
    pub http_headers_size: Option<i64>,
    #[serde(default, rename = "http_headers_size_prev")]
    pub http_headers_size_prev: Option<i64>,
    #[serde(default, rename = "images_no_crawl_reason")]
    pub images_no_crawl_reason: Option<Vec<String>>,
    #[serde(default, rename = "images_no_crawl_reason_prev")]
    pub images_no_crawl_reason_prev: Option<Vec<String>>,
    #[serde(default, rename = "incoming_all_links")]
    pub incoming_all_links: Option<i64>,
    #[serde(default, rename = "incoming_all_links_diff")]
    pub incoming_all_links_diff: Option<i64>,
    #[serde(default, rename = "incoming_all_links_prev")]
    pub incoming_all_links_prev: Option<i64>,
    #[serde(default, rename = "incoming_canonical")]
    pub incoming_canonical: Option<i64>,
    #[serde(default, rename = "incoming_canonical_prev")]
    pub incoming_canonical_prev: Option<i64>,
    #[serde(default, rename = "incoming_css")]
    pub incoming_css: Option<i64>,
    #[serde(default, rename = "incoming_css_prev")]
    pub incoming_css_prev: Option<i64>,
    #[serde(default, rename = "incoming_follow")]
    pub incoming_follow: Option<i64>,
    #[serde(default, rename = "incoming_follow_prev")]
    pub incoming_follow_prev: Option<i64>,
    #[serde(default, rename = "incoming_hreflang")]
    pub incoming_hreflang: Option<i64>,
    #[serde(default, rename = "incoming_hreflang_prev")]
    pub incoming_hreflang_prev: Option<i64>,
    #[serde(default, rename = "incoming_image")]
    pub incoming_image: Option<i64>,
    #[serde(default, rename = "incoming_image_prev")]
    pub incoming_image_prev: Option<i64>,
    #[serde(default, rename = "incoming_js")]
    pub incoming_js: Option<i64>,
    #[serde(default, rename = "incoming_js_prev")]
    pub incoming_js_prev: Option<i64>,
    #[serde(default, rename = "incoming_links")]
    pub incoming_links: Option<i64>,
    #[serde(default, rename = "incoming_links_prev")]
    pub incoming_links_prev: Option<i64>,
    #[serde(default, rename = "incoming_nofollow")]
    pub incoming_nofollow: Option<i64>,
    #[serde(default, rename = "incoming_nofollow_prev")]
    pub incoming_nofollow_prev: Option<i64>,
    #[serde(default, rename = "incoming_pagination")]
    pub incoming_pagination: Option<i64>,
    #[serde(default, rename = "incoming_pagination_prev")]
    pub incoming_pagination_prev: Option<i64>,
    #[serde(default, rename = "incoming_redirect")]
    pub incoming_redirect: Option<i64>,
    #[serde(default, rename = "incoming_redirect_prev")]
    pub incoming_redirect_prev: Option<i64>,
    #[serde(default, rename = "indexnow_error")]
    pub indexnow_error: Option<String>,
    #[serde(default, rename = "indexnow_error_prev")]
    pub indexnow_error_prev: Option<String>,
    #[serde(default, rename = "indexnow_reason")]
    pub indexnow_reason: Option<String>,
    #[serde(default, rename = "indexnow_reason_prev")]
    pub indexnow_reason_prev: Option<String>,
    #[serde(default, rename = "indexnow_status")]
    pub indexnow_status: Option<String>,
    #[serde(default, rename = "indexnow_status_prev")]
    pub indexnow_status_prev: Option<String>,
    #[serde(default, rename = "indexnow_submitted_at")]
    pub indexnow_submitted_at: Option<String>,
    #[serde(default, rename = "internal")]
    pub internal: Option<bool>,
    #[serde(default, rename = "internal_code")]
    pub internal_code: Option<Vec<i64>>,
    #[serde(default, rename = "internal_code_prev")]
    pub internal_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "internal_link_anchor")]
    pub internal_link_anchor: Option<Vec<String>>,
    #[serde(default, rename = "internal_link_anchor_prev")]
    pub internal_link_anchor_prev: Option<Vec<String>>,
    #[serde(default, rename = "internal_link_domain")]
    pub internal_link_domain: Option<Vec<String>>,
    #[serde(default, rename = "internal_link_domain_prev")]
    pub internal_link_domain_prev: Option<Vec<String>>,
    #[serde(default, rename = "internal_links")]
    pub internal_links: Option<Vec<String>>,
    #[serde(default, rename = "internal_links_is_canonical")]
    pub internal_links_is_canonical: Option<Vec<bool>>,
    #[serde(default, rename = "internal_links_is_canonical_prev")]
    pub internal_links_is_canonical_prev: Option<Vec<bool>>,
    #[serde(default, rename = "internal_links_prev")]
    pub internal_links_prev: Option<Vec<String>>,
    #[serde(default, rename = "internal_no_crawl_reason")]
    pub internal_no_crawl_reason: Option<Vec<String>>,
    #[serde(default, rename = "internal_no_crawl_reason_prev")]
    pub internal_no_crawl_reason_prev: Option<Vec<String>>,
    #[serde(default, rename = "internal_scheme")]
    pub internal_scheme: Option<Vec<String>>,
    #[serde(default, rename = "internal_scheme_prev")]
    pub internal_scheme_prev: Option<Vec<String>>,
    #[serde(default, rename = "is_html")]
    pub is_html: Option<bool>,
    #[serde(default, rename = "is_html_prev")]
    pub is_html_prev: Option<bool>,
    #[serde(default, rename = "is_in_sitemap")]
    pub is_in_sitemap: Option<bool>,
    #[serde(default, rename = "is_in_sitemap_prev")]
    pub is_in_sitemap_prev: Option<bool>,
    #[serde(default, rename = "is_page_title_used_in_serp")]
    pub is_page_title_used_in_serp: Option<bool>,
    #[serde(default, rename = "is_redirect_loop")]
    pub is_redirect_loop: Option<bool>,
    #[serde(default, rename = "is_redirect_loop_prev")]
    pub is_redirect_loop_prev: Option<bool>,
    #[serde(default, rename = "is_rendered")]
    pub is_rendered: Option<bool>,
    #[serde(default, rename = "is_rendered_prev")]
    pub is_rendered_prev: Option<bool>,
    #[serde(default, rename = "is_valid_internal_html")]
    pub is_valid_internal_html: Option<bool>,
    #[serde(default, rename = "is_valid_internal_html_prev")]
    pub is_valid_internal_html_prev: Option<bool>,
    #[serde(default, rename = "js_no_crawl_reason")]
    pub js_no_crawl_reason: Option<Vec<String>>,
    #[serde(default, rename = "js_no_crawl_reason_prev")]
    pub js_no_crawl_reason_prev: Option<Vec<String>>,
    #[serde(default, rename = "jsonld_schema_types")]
    pub jsonld_schema_types: Option<Vec<String>>,
    #[serde(default, rename = "jsonld_schema_types_prev")]
    pub jsonld_schema_types_prev: Option<Vec<String>>,
    #[serde(default, rename = "jsonld_type_properties")]
    pub jsonld_type_properties: Option<Vec<String>>,
    #[serde(default, rename = "jsonld_type_properties_prev")]
    pub jsonld_type_properties_prev: Option<Vec<String>>,
    #[serde(default, rename = "jsonld_validation_kinds")]
    pub jsonld_validation_kinds: Option<Vec<String>>,
    #[serde(default, rename = "jsonld_validation_kinds_prev")]
    pub jsonld_validation_kinds_prev: Option<Vec<String>>,
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<String>>,
    #[serde(default, rename = "keywords_prev")]
    pub keywords_prev: Option<Vec<String>>,
    #[serde(default, rename = "length")]
    pub length: Option<i64>,
    #[serde(default, rename = "links_count_css")]
    pub links_count_css: Option<i64>,
    #[serde(default, rename = "links_count_css_prev")]
    pub links_count_css_prev: Option<i64>,
    #[serde(default, rename = "links_count_external")]
    pub links_count_external: Option<i64>,
    #[serde(default, rename = "links_count_external3xx")]
    pub links_count_external3xx: Option<i64>,
    #[serde(default, rename = "links_count_external3xx_prev")]
    pub links_count_external3xx_prev: Option<i64>,
    #[serde(default, rename = "links_count_external4xx")]
    pub links_count_external4xx: Option<i64>,
    #[serde(default, rename = "links_count_external4xx_prev")]
    pub links_count_external4xx_prev: Option<i64>,
    #[serde(default, rename = "links_count_external5xx")]
    pub links_count_external5xx: Option<i64>,
    #[serde(default, rename = "links_count_external5xx_prev")]
    pub links_count_external5xx_prev: Option<i64>,
    #[serde(default, rename = "links_count_external_follow")]
    pub links_count_external_follow: Option<i64>,
    #[serde(default, rename = "links_count_external_follow_prev")]
    pub links_count_external_follow_prev: Option<i64>,
    #[serde(default, rename = "links_count_external_nofollow")]
    pub links_count_external_nofollow: Option<i64>,
    #[serde(default, rename = "links_count_external_nofollow_prev")]
    pub links_count_external_nofollow_prev: Option<i64>,
    #[serde(default, rename = "links_count_external_non_canonical")]
    pub links_count_external_non_canonical: Option<i64>,
    #[serde(default, rename = "links_count_external_non_canonical_prev")]
    pub links_count_external_non_canonical_prev: Option<i64>,
    #[serde(default, rename = "links_count_external_xxx")]
    pub links_count_external_xxx: Option<i64>,
    #[serde(default, rename = "links_count_external_xxx_prev")]
    pub links_count_external_xxx_prev: Option<i64>,
    #[serde(default, rename = "links_count_images")]
    pub links_count_images: Option<i64>,
    #[serde(default, rename = "links_count_images_prev")]
    pub links_count_images_prev: Option<i64>,
    #[serde(default, rename = "links_count_images_with_alt")]
    pub links_count_images_with_alt: Option<i64>,
    #[serde(default, rename = "links_count_images_with_alt_prev")]
    pub links_count_images_with_alt_prev: Option<i64>,
    #[serde(default, rename = "links_count_images_without_alt")]
    pub links_count_images_without_alt: Option<i64>,
    #[serde(default, rename = "links_count_images_without_alt_prev")]
    pub links_count_images_without_alt_prev: Option<i64>,
    #[serde(default, rename = "links_count_internal")]
    pub links_count_internal: Option<i64>,
    #[serde(default, rename = "links_count_internal3xx")]
    pub links_count_internal3xx: Option<i64>,
    #[serde(default, rename = "links_count_internal3xx_prev")]
    pub links_count_internal3xx_prev: Option<i64>,
    #[serde(default, rename = "links_count_internal4xx")]
    pub links_count_internal4xx: Option<i64>,
    #[serde(default, rename = "links_count_internal4xx_prev")]
    pub links_count_internal4xx_prev: Option<i64>,
    #[serde(default, rename = "links_count_internal5xx")]
    pub links_count_internal5xx: Option<i64>,
    #[serde(default, rename = "links_count_internal5xx_prev")]
    pub links_count_internal5xx_prev: Option<i64>,
    #[serde(default, rename = "links_count_internal_follow")]
    pub links_count_internal_follow: Option<i64>,
    #[serde(default, rename = "links_count_internal_follow_prev")]
    pub links_count_internal_follow_prev: Option<i64>,
    #[serde(default, rename = "links_count_internal_nofollow")]
    pub links_count_internal_nofollow: Option<i64>,
    #[serde(default, rename = "links_count_internal_nofollow_prev")]
    pub links_count_internal_nofollow_prev: Option<i64>,
    #[serde(default, rename = "links_count_internal_non_canonical")]
    pub links_count_internal_non_canonical: Option<i64>,
    #[serde(default, rename = "links_count_internal_non_canonical_prev")]
    pub links_count_internal_non_canonical_prev: Option<i64>,
    #[serde(default, rename = "links_count_internal_xxx")]
    pub links_count_internal_xxx: Option<i64>,
    #[serde(default, rename = "links_count_internal_xxx_prev")]
    pub links_count_internal_xxx_prev: Option<i64>,
    #[serde(default, rename = "links_count_js")]
    pub links_count_js: Option<i64>,
    #[serde(default, rename = "links_count_js_prev")]
    pub links_count_js_prev: Option<i64>,
    #[serde(default, rename = "links_css")]
    pub links_css: Option<Vec<String>>,
    #[serde(default, rename = "links_css_code")]
    pub links_css_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_css_code_prev")]
    pub links_css_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_css_domain")]
    pub links_css_domain: Option<Vec<String>>,
    #[serde(default, rename = "links_css_domain_prev")]
    pub links_css_domain_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_css_prev")]
    pub links_css_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_css_scheme")]
    pub links_css_scheme: Option<Vec<String>>,
    #[serde(default, rename = "links_css_scheme_prev")]
    pub links_css_scheme_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_external3xx")]
    pub links_external3xx: Option<Vec<String>>,
    #[serde(default, rename = "links_external3xx_code")]
    pub links_external3xx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_external3xx_code_prev")]
    pub links_external3xx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_external3xx_prev")]
    pub links_external3xx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_external4xx")]
    pub links_external4xx: Option<Vec<String>>,
    #[serde(default, rename = "links_external4xx_code")]
    pub links_external4xx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_external4xx_code_prev")]
    pub links_external4xx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_external4xx_prev")]
    pub links_external4xx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_external5xx")]
    pub links_external5xx: Option<Vec<String>>,
    #[serde(default, rename = "links_external5xx_code")]
    pub links_external5xx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_external5xx_code_prev")]
    pub links_external5xx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_external5xx_prev")]
    pub links_external5xx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_external_follow")]
    pub links_external_follow: Option<Vec<String>>,
    #[serde(default, rename = "links_external_follow_code")]
    pub links_external_follow_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_follow_code_prev")]
    pub links_external_follow_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_follow_prev")]
    pub links_external_follow_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_external_nofollow")]
    pub links_external_nofollow: Option<Vec<String>>,
    #[serde(default, rename = "links_external_nofollow_code")]
    pub links_external_nofollow_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_nofollow_code_prev")]
    pub links_external_nofollow_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_nofollow_prev")]
    pub links_external_nofollow_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_external_non_canonical")]
    pub links_external_non_canonical: Option<Vec<String>>,
    #[serde(default, rename = "links_external_non_canonical_code")]
    pub links_external_non_canonical_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_non_canonical_code_prev")]
    pub links_external_non_canonical_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_non_canonical_prev")]
    pub links_external_non_canonical_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_external_xxx")]
    pub links_external_xxx: Option<Vec<String>>,
    #[serde(default, rename = "links_external_xxx_code")]
    pub links_external_xxx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_xxx_code_prev")]
    pub links_external_xxx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_external_xxx_prev")]
    pub links_external_xxx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_hreflang_code")]
    pub links_hreflang_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_hreflang_code_prev")]
    pub links_hreflang_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_images")]
    pub links_images: Option<Vec<String>>,
    #[serde(default, rename = "links_images_alt")]
    pub links_images_alt: Option<Vec<String>>,
    #[serde(default, rename = "links_images_alt_prev")]
    pub links_images_alt_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_images_code")]
    pub links_images_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_images_code_prev")]
    pub links_images_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_images_domain")]
    pub links_images_domain: Option<Vec<String>>,
    #[serde(default, rename = "links_images_domain_prev")]
    pub links_images_domain_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_images_prev")]
    pub links_images_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_images_scheme")]
    pub links_images_scheme: Option<Vec<String>>,
    #[serde(default, rename = "links_images_scheme_prev")]
    pub links_images_scheme_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_images_with_alt")]
    pub links_images_with_alt: Option<Vec<String>>,
    #[serde(default, rename = "links_images_with_alt_code")]
    pub links_images_with_alt_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_images_with_alt_code_prev")]
    pub links_images_with_alt_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_images_with_alt_prev")]
    pub links_images_with_alt_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_images_without_alt")]
    pub links_images_without_alt: Option<Vec<String>>,
    #[serde(default, rename = "links_images_without_alt_code")]
    pub links_images_without_alt_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_images_without_alt_code_prev")]
    pub links_images_without_alt_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_images_without_alt_prev")]
    pub links_images_without_alt_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_internal3xx")]
    pub links_internal3xx: Option<Vec<String>>,
    #[serde(default, rename = "links_internal3xx_code")]
    pub links_internal3xx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal3xx_code_prev")]
    pub links_internal3xx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal3xx_prev")]
    pub links_internal3xx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_internal4xx")]
    pub links_internal4xx: Option<Vec<String>>,
    #[serde(default, rename = "links_internal4xx_code")]
    pub links_internal4xx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal4xx_code_prev")]
    pub links_internal4xx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal4xx_prev")]
    pub links_internal4xx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_internal5xx")]
    pub links_internal5xx: Option<Vec<String>>,
    #[serde(default, rename = "links_internal5xx_code")]
    pub links_internal5xx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal5xx_code_prev")]
    pub links_internal5xx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal5xx_prev")]
    pub links_internal5xx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_follow")]
    pub links_internal_follow: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_follow_code")]
    pub links_internal_follow_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_follow_code_prev")]
    pub links_internal_follow_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_follow_prev")]
    pub links_internal_follow_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_nofollow")]
    pub links_internal_nofollow: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_nofollow_code")]
    pub links_internal_nofollow_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_nofollow_code_prev")]
    pub links_internal_nofollow_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_nofollow_prev")]
    pub links_internal_nofollow_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_non_canonical")]
    pub links_internal_non_canonical: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_non_canonical_code")]
    pub links_internal_non_canonical_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_non_canonical_code_prev")]
    pub links_internal_non_canonical_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_non_canonical_prev")]
    pub links_internal_non_canonical_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_xxx")]
    pub links_internal_xxx: Option<Vec<String>>,
    #[serde(default, rename = "links_internal_xxx_code")]
    pub links_internal_xxx_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_xxx_code_prev")]
    pub links_internal_xxx_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_internal_xxx_prev")]
    pub links_internal_xxx_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_js")]
    pub links_js: Option<Vec<String>>,
    #[serde(default, rename = "links_js_code")]
    pub links_js_code: Option<Vec<i64>>,
    #[serde(default, rename = "links_js_code_prev")]
    pub links_js_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "links_js_domain")]
    pub links_js_domain: Option<Vec<String>>,
    #[serde(default, rename = "links_js_domain_prev")]
    pub links_js_domain_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_js_prev")]
    pub links_js_prev: Option<Vec<String>>,
    #[serde(default, rename = "links_js_scheme")]
    pub links_js_scheme: Option<Vec<String>>,
    #[serde(default, rename = "links_js_scheme_prev")]
    pub links_js_scheme_prev: Option<Vec<String>>,
    #[serde(default, rename = "loading_time")]
    pub loading_time: Option<i64>,
    #[serde(default, rename = "loading_time_diff")]
    pub loading_time_diff: Option<i64>,
    #[serde(default, rename = "loading_time_prev")]
    pub loading_time_prev: Option<i64>,
    #[serde(default, rename = "meta_description")]
    pub meta_description: Option<Vec<String>>,
    #[serde(default, rename = "meta_description_length")]
    pub meta_description_length: Option<Vec<i64>>,
    #[serde(default, rename = "meta_description_length_prev")]
    pub meta_description_length_prev: Option<Vec<i64>>,
    #[serde(default, rename = "meta_description_prev")]
    pub meta_description_prev: Option<Vec<String>>,
    #[serde(default, rename = "meta_refresh")]
    pub meta_refresh: Option<Vec<String>>,
    #[serde(default, rename = "meta_refresh_prev")]
    pub meta_refresh_prev: Option<Vec<String>>,
    #[serde(default, rename = "meta_robots")]
    pub meta_robots: Option<Vec<String>>,
    #[serde(default, rename = "meta_robots_prev")]
    pub meta_robots_prev: Option<Vec<String>>,
    #[serde(default, rename = "meta_twitter_tags_app_google_play")]
    pub meta_twitter_tags_app_google_play: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_app_google_play_prev")]
    pub meta_twitter_tags_app_google_play_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_app_ipad")]
    pub meta_twitter_tags_app_ipad: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_app_ipad_prev")]
    pub meta_twitter_tags_app_ipad_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_app_iphone")]
    pub meta_twitter_tags_app_iphone: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_app_iphone_prev")]
    pub meta_twitter_tags_app_iphone_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_attributes")]
    pub meta_twitter_tags_attributes: Option<Vec<String>>,
    #[serde(default, rename = "meta_twitter_tags_attributes_prev")]
    pub meta_twitter_tags_attributes_prev: Option<Vec<String>>,
    #[serde(default, rename = "meta_twitter_tags_card")]
    pub meta_twitter_tags_card: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_card_prev")]
    pub meta_twitter_tags_card_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_description")]
    pub meta_twitter_tags_description: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_description_prev")]
    pub meta_twitter_tags_description_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_image")]
    pub meta_twitter_tags_image: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_image_prev")]
    pub meta_twitter_tags_image_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_image_url_invalid")]
    pub meta_twitter_tags_image_url_invalid: Option<bool>,
    #[serde(default, rename = "meta_twitter_tags_image_url_invalid_prev")]
    pub meta_twitter_tags_image_url_invalid_prev: Option<bool>,
    #[serde(default, rename = "meta_twitter_tags_player")]
    pub meta_twitter_tags_player: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_player_height")]
    pub meta_twitter_tags_player_height: Option<i64>,
    #[serde(default, rename = "meta_twitter_tags_player_height_prev")]
    pub meta_twitter_tags_player_height_prev: Option<i64>,
    #[serde(default, rename = "meta_twitter_tags_player_prev")]
    pub meta_twitter_tags_player_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_player_width")]
    pub meta_twitter_tags_player_width: Option<i64>,
    #[serde(default, rename = "meta_twitter_tags_player_width_diff")]
    pub meta_twitter_tags_player_width_diff: Option<i64>,
    #[serde(default, rename = "meta_twitter_tags_player_width_prev")]
    pub meta_twitter_tags_player_width_prev: Option<i64>,
    #[serde(default, rename = "meta_twitter_tags_site")]
    pub meta_twitter_tags_site: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_site_prev")]
    pub meta_twitter_tags_site_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_title")]
    pub meta_twitter_tags_title: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_title_prev")]
    pub meta_twitter_tags_title_prev: Option<String>,
    #[serde(default, rename = "meta_twitter_tags_valid")]
    pub meta_twitter_tags_valid: Option<bool>,
    #[serde(default, rename = "meta_twitter_tags_valid_prev")]
    pub meta_twitter_tags_valid_prev: Option<bool>,
    #[serde(default, rename = "meta_twitter_tags_values")]
    pub meta_twitter_tags_values: Option<Vec<String>>,
    #[serde(default, rename = "meta_twitter_tags_values_prev")]
    pub meta_twitter_tags_values_prev: Option<Vec<String>>,
    #[serde(default, rename = "ml_page_types")]
    pub ml_page_types: Option<String>,
    #[serde(default, rename = "ml_page_types_prev")]
    pub ml_page_types_prev: Option<String>,
    #[serde(default, rename = "navigation_next")]
    pub navigation_next: Option<String>,
    #[serde(default, rename = "navigation_next_code")]
    pub navigation_next_code: Option<i64>,
    #[serde(default, rename = "navigation_next_code_prev")]
    pub navigation_next_code_prev: Option<i64>,
    #[serde(default, rename = "navigation_next_no_crawl_reason")]
    pub navigation_next_no_crawl_reason: Option<String>,
    #[serde(default, rename = "navigation_next_no_crawl_reason_prev")]
    pub navigation_next_no_crawl_reason_prev: Option<String>,
    #[serde(default, rename = "navigation_next_prev")]
    pub navigation_next_prev: Option<String>,
    #[serde(default, rename = "navigation_prev")]
    pub navigation_prev: Option<String>,
    #[serde(default, rename = "navigation_prev_code")]
    pub navigation_prev_code: Option<i64>,
    #[serde(default, rename = "navigation_prev_code_prev")]
    pub navigation_prev_code_prev: Option<i64>,
    #[serde(default, rename = "navigation_prev_no_crawl_reason")]
    pub navigation_prev_no_crawl_reason: Option<String>,
    #[serde(default, rename = "navigation_prev_no_crawl_reason_prev")]
    pub navigation_prev_no_crawl_reason_prev: Option<String>,
    #[serde(default, rename = "navigation_prev_prev")]
    pub navigation_prev_prev: Option<String>,
    #[serde(default, rename = "nofollow")]
    pub nofollow: Option<i64>,
    #[serde(default, rename = "nofollow_diff")]
    pub nofollow_diff: Option<i64>,
    #[serde(default, rename = "nofollow_prev")]
    pub nofollow_prev: Option<i64>,
    #[serde(default, rename = "nr_h1")]
    pub nr_h1: Option<i64>,
    #[serde(default, rename = "nr_h1_diff")]
    pub nr_h1_diff: Option<i64>,
    #[serde(default, rename = "nr_h1_prev")]
    pub nr_h1_prev: Option<i64>,
    #[serde(default, rename = "nr_meta_description")]
    pub nr_meta_description: Option<i64>,
    #[serde(default, rename = "nr_meta_description_diff")]
    pub nr_meta_description_diff: Option<i64>,
    #[serde(default, rename = "nr_meta_description_prev")]
    pub nr_meta_description_prev: Option<i64>,
    #[serde(default, rename = "nr_redirect_chain_urls")]
    pub nr_redirect_chain_urls: Option<i64>,
    #[serde(default, rename = "nr_redirect_chain_urls_diff")]
    pub nr_redirect_chain_urls_diff: Option<i64>,
    #[serde(default, rename = "nr_redirect_chain_urls_prev")]
    pub nr_redirect_chain_urls_prev: Option<i64>,
    #[serde(default, rename = "nr_titles")]
    pub nr_titles: Option<i64>,
    #[serde(default, rename = "nr_titles_diff")]
    pub nr_titles_diff: Option<i64>,
    #[serde(default, rename = "nr_titles_prev")]
    pub nr_titles_prev: Option<i64>,
    #[serde(default, rename = "og_tags_attributes")]
    pub og_tags_attributes: Option<Vec<String>>,
    #[serde(default, rename = "og_tags_attributes_prev")]
    pub og_tags_attributes_prev: Option<Vec<String>>,
    #[serde(default, rename = "og_tags_image")]
    pub og_tags_image: Option<String>,
    #[serde(default, rename = "og_tags_image_prev")]
    pub og_tags_image_prev: Option<String>,
    #[serde(default, rename = "og_tags_image_url_invalid")]
    pub og_tags_image_url_invalid: Option<bool>,
    #[serde(default, rename = "og_tags_image_url_invalid_prev")]
    pub og_tags_image_url_invalid_prev: Option<bool>,
    #[serde(default, rename = "og_tags_inconsistent_canonical")]
    pub og_tags_inconsistent_canonical: Option<bool>,
    #[serde(default, rename = "og_tags_inconsistent_canonical_prev")]
    pub og_tags_inconsistent_canonical_prev: Option<bool>,
    #[serde(default, rename = "og_tags_title")]
    pub og_tags_title: Option<String>,
    #[serde(default, rename = "og_tags_title_prev")]
    pub og_tags_title_prev: Option<String>,
    #[serde(default, rename = "og_tags_type")]
    pub og_tags_type: Option<String>,
    #[serde(default, rename = "og_tags_type_prev")]
    pub og_tags_type_prev: Option<String>,
    #[serde(default, rename = "og_tags_url")]
    pub og_tags_url: Option<String>,
    #[serde(default, rename = "og_tags_url_prev")]
    pub og_tags_url_prev: Option<String>,
    #[serde(default, rename = "og_tags_url_valid")]
    pub og_tags_url_valid: Option<bool>,
    #[serde(default, rename = "og_tags_url_valid_prev")]
    pub og_tags_url_valid_prev: Option<bool>,
    #[serde(default, rename = "og_tags_valid")]
    pub og_tags_valid: Option<bool>,
    #[serde(default, rename = "og_tags_valid_prev")]
    pub og_tags_valid_prev: Option<bool>,
    #[serde(default, rename = "og_tags_value")]
    pub og_tags_value: Option<Vec<String>>,
    #[serde(default, rename = "og_tags_value_prev")]
    pub og_tags_value_prev: Option<Vec<String>>,
    #[serde(default, rename = "origin")]
    pub origin: Option<String>,
    #[serde(default, rename = "origin_prev")]
    pub origin_prev: Option<String>,
    #[serde(default, rename = "page_any_content_raw_length")]
    pub page_any_content_raw_length: Option<i64>,
    #[serde(default, rename = "page_any_content_raw_length_diff")]
    pub page_any_content_raw_length_diff: Option<i64>,
    #[serde(default, rename = "page_any_content_raw_length_prev")]
    pub page_any_content_raw_length_prev: Option<i64>,
    #[serde(default, rename = "page_is_nofollow")]
    pub page_is_nofollow: Option<bool>,
    #[serde(default, rename = "page_is_nofollow_prev")]
    pub page_is_nofollow_prev: Option<bool>,
    #[serde(default, rename = "page_is_noindex")]
    pub page_is_noindex: Option<bool>,
    #[serde(default, rename = "page_is_noindex_prev")]
    pub page_is_noindex_prev: Option<bool>,
    #[serde(default, rename = "page_rating")]
    pub page_rating: Option<i64>,
    #[serde(default, rename = "page_raw_text_length")]
    pub page_raw_text_length: Option<i64>,
    #[serde(default, rename = "page_raw_text_length_diff")]
    pub page_raw_text_length_diff: Option<i64>,
    #[serde(default, rename = "page_raw_text_length_prev")]
    pub page_raw_text_length_prev: Option<i64>,
    #[serde(default, rename = "page_raw_ur")]
    pub page_raw_ur: Option<i64>,
    #[serde(default, rename = "page_raw_ur_diff")]
    pub page_raw_ur_diff: Option<i64>,
    #[serde(default, rename = "page_raw_ur_prev")]
    pub page_raw_ur_prev: Option<i64>,
    #[serde(default, rename = "page_type")]
    pub page_type: Option<Vec<String>>,
    #[serde(default, rename = "page_type_prev")]
    pub page_type_prev: Option<Vec<String>>,
    #[serde(default, rename = "pagination_group")]
    pub pagination_group: Option<i64>,
    #[serde(default, rename = "pagination_group_prev")]
    pub pagination_group_prev: Option<i64>,
    #[serde(default, rename = "positions")]
    pub positions: Option<i64>,
    #[serde(default, rename = "positions_diff")]
    pub positions_diff: Option<i64>,
    #[serde(default, rename = "positions_prev")]
    pub positions_prev: Option<i64>,
    #[serde(default, rename = "positions_top3")]
    pub positions_top3: Option<i64>,
    #[serde(default, rename = "positions_top3_diff")]
    pub positions_top3_diff: Option<i64>,
    #[serde(default, rename = "positions_top3_prev")]
    pub positions_top3_prev: Option<i64>,
    #[serde(default, rename = "positions_top10")]
    pub positions_top10: Option<i64>,
    #[serde(default, rename = "positions_top10_diff")]
    pub positions_top10_diff: Option<i64>,
    #[serde(default, rename = "positions_top10_prev")]
    pub positions_top10_prev: Option<i64>,
    #[serde(default, rename = "psi_crux_cls_category")]
    pub psi_crux_cls_category: Option<String>,
    #[serde(default, rename = "psi_crux_cls_category_prev")]
    pub psi_crux_cls_category_prev: Option<String>,
    #[serde(default, rename = "psi_crux_cls_distributions_proportion")]
    pub psi_crux_cls_distributions_proportion: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_cls_distributions_proportion_prev")]
    pub psi_crux_cls_distributions_proportion_prev: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_cls_percentile")]
    pub psi_crux_cls_percentile: Option<f64>,
    #[serde(default, rename = "psi_crux_cls_percentile_diff")]
    pub psi_crux_cls_percentile_diff: Option<f64>,
    #[serde(default, rename = "psi_crux_cls_percentile_prev")]
    pub psi_crux_cls_percentile_prev: Option<f64>,
    #[serde(default, rename = "psi_crux_fid_category")]
    pub psi_crux_fid_category: Option<String>,
    #[serde(default, rename = "psi_crux_fid_category_prev")]
    pub psi_crux_fid_category_prev: Option<String>,
    #[serde(default, rename = "psi_crux_fid_distributions_proportion")]
    pub psi_crux_fid_distributions_proportion: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_fid_distributions_proportion_prev")]
    pub psi_crux_fid_distributions_proportion_prev: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_fid_percentile")]
    pub psi_crux_fid_percentile: Option<f64>,
    #[serde(default, rename = "psi_crux_fid_percentile_prev")]
    pub psi_crux_fid_percentile_prev: Option<f64>,
    #[serde(default, rename = "psi_crux_inp_category")]
    pub psi_crux_inp_category: Option<String>,
    #[serde(default, rename = "psi_crux_inp_category_prev")]
    pub psi_crux_inp_category_prev: Option<String>,
    #[serde(default, rename = "psi_crux_inp_distributions_proportion")]
    pub psi_crux_inp_distributions_proportion: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_inp_distributions_proportion_prev")]
    pub psi_crux_inp_distributions_proportion_prev: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_inp_percentile")]
    pub psi_crux_inp_percentile: Option<f64>,
    #[serde(default, rename = "psi_crux_inp_percentile_diff")]
    pub psi_crux_inp_percentile_diff: Option<f64>,
    #[serde(default, rename = "psi_crux_inp_percentile_prev")]
    pub psi_crux_inp_percentile_prev: Option<f64>,
    #[serde(default, rename = "psi_crux_lcp_category")]
    pub psi_crux_lcp_category: Option<String>,
    #[serde(default, rename = "psi_crux_lcp_category_prev")]
    pub psi_crux_lcp_category_prev: Option<String>,
    #[serde(default, rename = "psi_crux_lcp_distributions_proportion")]
    pub psi_crux_lcp_distributions_proportion: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_lcp_distributions_proportion_prev")]
    pub psi_crux_lcp_distributions_proportion_prev: Option<Vec<f64>>,
    #[serde(default, rename = "psi_crux_lcp_percentile")]
    pub psi_crux_lcp_percentile: Option<f64>,
    #[serde(default, rename = "psi_crux_lcp_percentile_diff")]
    pub psi_crux_lcp_percentile_diff: Option<f64>,
    #[serde(default, rename = "psi_crux_lcp_percentile_prev")]
    pub psi_crux_lcp_percentile_prev: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_cls_error_message")]
    pub psi_lighthouse_cls_error_message: Option<String>,
    #[serde(default, rename = "psi_lighthouse_cls_error_message_prev")]
    pub psi_lighthouse_cls_error_message_prev: Option<String>,
    #[serde(default, rename = "psi_lighthouse_cls_value")]
    pub psi_lighthouse_cls_value: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_cls_value_diff")]
    pub psi_lighthouse_cls_value_diff: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_cls_value_prev")]
    pub psi_lighthouse_cls_value_prev: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_lcp_error_message")]
    pub psi_lighthouse_lcp_error_message: Option<String>,
    #[serde(default, rename = "psi_lighthouse_lcp_error_message_prev")]
    pub psi_lighthouse_lcp_error_message_prev: Option<String>,
    #[serde(default, rename = "psi_lighthouse_lcp_value")]
    pub psi_lighthouse_lcp_value: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_lcp_value_diff")]
    pub psi_lighthouse_lcp_value_diff: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_lcp_value_prev")]
    pub psi_lighthouse_lcp_value_prev: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_score")]
    pub psi_lighthouse_score: Option<i64>,
    #[serde(default, rename = "psi_lighthouse_score_prev")]
    pub psi_lighthouse_score_prev: Option<i64>,
    #[serde(default, rename = "psi_lighthouse_tbt_error_message")]
    pub psi_lighthouse_tbt_error_message: Option<String>,
    #[serde(default, rename = "psi_lighthouse_tbt_error_message_prev")]
    pub psi_lighthouse_tbt_error_message_prev: Option<String>,
    #[serde(default, rename = "psi_lighthouse_tbt_value")]
    pub psi_lighthouse_tbt_value: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_tbt_value_diff")]
    pub psi_lighthouse_tbt_value_diff: Option<f64>,
    #[serde(default, rename = "psi_lighthouse_tbt_value_prev")]
    pub psi_lighthouse_tbt_value_prev: Option<f64>,
    #[serde(default, rename = "psi_mobile_issues")]
    pub psi_mobile_issues: Option<Vec<String>>,
    #[serde(default, rename = "psi_mobile_issues_explanations")]
    pub psi_mobile_issues_explanations: Option<Vec<String>>,
    #[serde(default, rename = "psi_mobile_issues_explanations_prev")]
    pub psi_mobile_issues_explanations_prev: Option<Vec<String>>,
    #[serde(default, rename = "psi_mobile_issues_prev")]
    pub psi_mobile_issues_prev: Option<Vec<String>>,
    #[serde(default, rename = "psi_request_error_message")]
    pub psi_request_error_message: Option<String>,
    #[serde(default, rename = "psi_request_status")]
    pub psi_request_status: Option<String>,
    #[serde(default, rename = "redirect")]
    pub redirect: Option<String>,
    #[serde(default, rename = "redirect_chain_urls")]
    pub redirect_chain_urls: Option<Vec<String>>,
    #[serde(default, rename = "redirect_chain_urls_code")]
    pub redirect_chain_urls_code: Option<Vec<i64>>,
    #[serde(default, rename = "redirect_chain_urls_code_prev")]
    pub redirect_chain_urls_code_prev: Option<Vec<i64>>,
    #[serde(default, rename = "redirect_chain_urls_no_crawl_reason")]
    pub redirect_chain_urls_no_crawl_reason: Option<Vec<String>>,
    #[serde(default, rename = "redirect_chain_urls_no_crawl_reason_prev")]
    pub redirect_chain_urls_no_crawl_reason_prev: Option<Vec<String>>,
    #[serde(default, rename = "redirect_chain_urls_prev")]
    pub redirect_chain_urls_prev: Option<Vec<String>>,
    #[serde(default, rename = "redirect_code")]
    pub redirect_code: Option<i64>,
    #[serde(default, rename = "redirect_code_prev")]
    pub redirect_code_prev: Option<i64>,
    #[serde(default, rename = "redirect_counts")]
    pub redirect_counts: Option<i64>,
    #[serde(default, rename = "redirect_counts_diff")]
    pub redirect_counts_diff: Option<i64>,
    #[serde(default, rename = "redirect_counts_prev")]
    pub redirect_counts_prev: Option<i64>,
    #[serde(default, rename = "redirect_is_canonical")]
    pub redirect_is_canonical: Option<bool>,
    #[serde(default, rename = "redirect_is_canonical_prev")]
    pub redirect_is_canonical_prev: Option<bool>,
    #[serde(default, rename = "redirect_no_crawl_reason")]
    pub redirect_no_crawl_reason: Option<String>,
    #[serde(default, rename = "redirect_no_crawl_reason_prev")]
    pub redirect_no_crawl_reason_prev: Option<String>,
    #[serde(default, rename = "redirect_prev")]
    pub redirect_prev: Option<String>,
    #[serde(default, rename = "redirect_scheme")]
    pub redirect_scheme: Option<String>,
    #[serde(default, rename = "redirect_scheme_prev")]
    pub redirect_scheme_prev: Option<String>,
    #[serde(default, rename = "refclass_c")]
    pub refclass_c: Option<i64>,
    #[serde(default, rename = "refclass_c_diff")]
    pub refclass_c_diff: Option<i64>,
    #[serde(default, rename = "refclass_c_prev")]
    pub refclass_c_prev: Option<i64>,
    #[serde(default, rename = "refhosts")]
    pub refhosts: Option<i64>,
    #[serde(default, rename = "refhosts_diff")]
    pub refhosts_diff: Option<i64>,
    #[serde(default, rename = "refhosts_prev")]
    pub refhosts_prev: Option<i64>,
    #[serde(default, rename = "refips")]
    pub refips: Option<i64>,
    #[serde(default, rename = "refips_diff")]
    pub refips_diff: Option<i64>,
    #[serde(default, rename = "refips_prev")]
    pub refips_prev: Option<i64>,
    #[serde(default, rename = "refpages")]
    pub refpages: Option<i64>,
    #[serde(default, rename = "refpages_diff")]
    pub refpages_diff: Option<i64>,
    #[serde(default, rename = "refpages_prev")]
    pub refpages_prev: Option<i64>,
    #[serde(default, rename = "robots_error")]
    pub robots_error: Option<String>,
    #[serde(default, rename = "robots_error_prev")]
    pub robots_error_prev: Option<String>,
    #[serde(default, rename = "robots_sitemaps")]
    pub robots_sitemaps: Option<Vec<String>>,
    #[serde(default, rename = "robots_sitemaps_prev")]
    pub robots_sitemaps_prev: Option<Vec<String>>,
    #[serde(default, rename = "rss")]
    pub rss: Option<i64>,
    #[serde(default, rename = "rss_diff")]
    pub rss_diff: Option<i64>,
    #[serde(default, rename = "rss_prev")]
    pub rss_prev: Option<i64>,
    #[serde(default, rename = "scheme")]
    pub scheme: Option<String>,
    #[serde(default, rename = "self_canonical")]
    pub self_canonical: Option<bool>,
    #[serde(default, rename = "self_canonical_prev")]
    pub self_canonical_prev: Option<bool>,
    #[serde(default, rename = "self_hreflang")]
    pub self_hreflang: Option<Vec<String>>,
    #[serde(default, rename = "self_hreflang_code_is_valid")]
    pub self_hreflang_code_is_valid: Option<Vec<bool>>,
    #[serde(default, rename = "self_hreflang_code_is_valid_prev")]
    pub self_hreflang_code_is_valid_prev: Option<Vec<bool>>,
    #[serde(default, rename = "self_hreflang_country")]
    pub self_hreflang_country: Option<Vec<String>>,
    #[serde(default, rename = "self_hreflang_country_prev")]
    pub self_hreflang_country_prev: Option<Vec<String>>,
    #[serde(default, rename = "self_hreflang_language")]
    pub self_hreflang_language: Option<Vec<String>>,
    #[serde(default, rename = "self_hreflang_language_prev")]
    pub self_hreflang_language_prev: Option<Vec<String>>,
    #[serde(default, rename = "self_hreflang_prev")]
    pub self_hreflang_prev: Option<Vec<String>>,
    #[serde(default, rename = "serp_title")]
    pub serp_title: Option<String>,
    #[serde(default, rename = "serp_title_prev")]
    pub serp_title_prev: Option<String>,
    #[serde(default, rename = "sitemap_error")]
    pub sitemap_error: Option<String>,
    #[serde(default, rename = "sitemap_error_prev")]
    pub sitemap_error_prev: Option<String>,
    #[serde(default, rename = "sitemap_is_index")]
    pub sitemap_is_index: Option<bool>,
    #[serde(default, rename = "sitemap_is_index_prev")]
    pub sitemap_is_index_prev: Option<bool>,
    #[serde(default, rename = "sitemap_nr_urls")]
    pub sitemap_nr_urls: Option<i64>,
    #[serde(default, rename = "sitemap_nr_urls_diff")]
    pub sitemap_nr_urls_diff: Option<i64>,
    #[serde(default, rename = "sitemap_nr_urls_prev")]
    pub sitemap_nr_urls_prev: Option<i64>,
    #[serde(default, rename = "size")]
    pub size: Option<i64>,
    #[serde(default, rename = "size_prev")]
    pub size_prev: Option<i64>,
    #[serde(default, rename = "size_uncompressed")]
    pub size_uncompressed: Option<i64>,
    #[serde(default, rename = "size_uncompressed_diff")]
    pub size_uncompressed_diff: Option<i64>,
    #[serde(default, rename = "size_uncompressed_prev")]
    pub size_uncompressed_prev: Option<i64>,
    #[serde(default, rename = "source")]
    pub source: Option<Vec<String>>,
    #[serde(default, rename = "source_prev")]
    pub source_prev: Option<Vec<String>>,
    #[serde(default, rename = "stamp")]
    pub stamp: Option<String>,
    #[serde(default, rename = "stamp_prev")]
    pub stamp_prev: Option<String>,
    #[serde(default, rename = "time_to_first_byte")]
    pub time_to_first_byte: Option<i64>,
    #[serde(default, rename = "time_to_first_byte_diff")]
    pub time_to_first_byte_diff: Option<i64>,
    #[serde(default, rename = "time_to_first_byte_prev")]
    pub time_to_first_byte_prev: Option<i64>,
    #[serde(default, rename = "title")]
    pub title: Option<Vec<String>>,
    #[serde(default, rename = "title_prev")]
    pub title_prev: Option<Vec<String>>,
    #[serde(default, rename = "titles_length")]
    pub titles_length: Option<Vec<i64>>,
    #[serde(default, rename = "titles_length_prev")]
    pub titles_length_prev: Option<Vec<i64>>,
    #[serde(default, rename = "top_keyword")]
    pub top_keyword: Option<String>,
    #[serde(default, rename = "top_keyword_country")]
    pub top_keyword_country: Option<String>,
    #[serde(default, rename = "top_keyword_country_prev")]
    pub top_keyword_country_prev: Option<String>,
    #[serde(default, rename = "top_keyword_position")]
    pub top_keyword_position: Option<i64>,
    #[serde(default, rename = "top_keyword_position_diff")]
    pub top_keyword_position_diff: Option<i64>,
    #[serde(default, rename = "top_keyword_position_prev")]
    pub top_keyword_position_prev: Option<i64>,
    #[serde(default, rename = "traffic")]
    pub traffic: Option<f64>,
    #[serde(default, rename = "traffic_diff")]
    pub traffic_diff: Option<f64>,
    #[serde(default, rename = "traffic_prev")]
    pub traffic_prev: Option<f64>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PageExplorerResponse {
    #[serde(default, rename = "pages")]
    pub pages: Option<Vec<PagesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct HealthscoresItem {
    #[serde(default, rename = "date")]
    pub date: Option<String>,
    #[serde(default, rename = "health_score")]
    pub health_score: Option<i64>,
    #[serde(default, rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(default, rename = "project_name")]
    pub project_name: Option<String>,
    #[serde(default, rename = "status")]
    pub status: Option<String>,
    #[serde(default, rename = "target_mode")]
    pub target_mode: Option<String>,
    #[serde(default, rename = "target_protocol")]
    pub target_protocol: Option<String>,
    #[serde(default, rename = "target_url")]
    pub target_url: Option<String>,
    #[serde(default, rename = "total")]
    pub total: Option<i64>,
    #[serde(default, rename = "urls_with_errors")]
    pub urls_with_errors: Option<i64>,
    #[serde(default, rename = "urls_with_notices")]
    pub urls_with_notices: Option<i64>,
    #[serde(default, rename = "urls_with_warnings")]
    pub urls_with_warnings: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectsResponse {
    #[serde(default, rename = "healthscores")]
    pub healthscores: Option<Vec<HealthscoresItem>>,
}
