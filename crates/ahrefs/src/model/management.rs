//! Response models for the `management` API group.

#![allow(missing_docs)]

use serde_json::Value;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptsItem {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "prompt")]
    pub prompt: Option<String>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandRadarPromptsResponse {
    #[serde(default, rename = "prompts")]
    pub prompts: Option<Vec<PromptsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptsItem2 {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "prompt")]
    pub prompt: Option<String>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandRadarPromptsDeleteResponse {
    #[serde(default, rename = "prompts")]
    pub prompts: Option<Vec<PromptsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PromptsFrequencyItem {
    #[serde(default, rename = "data_source")]
    pub data_source: Option<String>,
    #[serde(default, rename = "frequency")]
    pub frequency: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UrlGroupsItem {
    #[serde(default, rename = "target")]
    pub target: Option<String>,
    #[serde(default, rename = "scope")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandsItem {
    #[serde(default, rename = "names")]
    pub names: Option<Vec<String>>,
    #[serde(default, rename = "url_groups")]
    pub url_groups: Option<Vec<UrlGroupsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UrlGroupsItem2 {
    #[serde(default, rename = "target")]
    pub target: Option<String>,
    #[serde(default, rename = "scope")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsItem {
    #[serde(default, rename = "names")]
    pub names: Option<Vec<String>>,
    #[serde(default, rename = "url_groups")]
    pub url_groups: Option<Vec<UrlGroupsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContentFilter {
    #[serde(default, rename = "include")]
    pub include: Option<Vec<String>>,
    #[serde(default, rename = "exclude")]
    pub exclude: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Settings {
    #[serde(default, rename = "brands")]
    pub brands: Option<Vec<BrandsItem>>,
    #[serde(default, rename = "competitors")]
    pub competitors: Option<Vec<CompetitorsItem>>,
    #[serde(default, rename = "country")]
    pub country: Option<Vec<String>>,
    #[serde(default, rename = "content_filter")]
    pub content_filter: Option<ContentFilter>,
    #[serde(default, rename = "where")]
    pub r#where: Option<Value>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReportsItem {
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "project_id")]
    pub project_id: Option<i64>,
    #[serde(default, rename = "prompts_frequency")]
    pub prompts_frequency: Option<Vec<PromptsFrequencyItem>>,
    #[serde(default, rename = "report_id")]
    pub report_id: Option<String>,
    #[serde(default, rename = "report_name")]
    pub report_name: Option<String>,
    #[serde(default, rename = "settings")]
    pub settings: Option<Settings>,
    #[serde(default, rename = "updated_at")]
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrandRadarReportsResponse {
    #[serde(default, rename = "reports")]
    pub reports: Option<Vec<ReportsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem {
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordListKeywordsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem2 {
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordListKeywordsDeleteResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordListsItem {
    #[serde(default, rename = "access")]
    pub access: Option<String>,
    #[serde(default, rename = "countries")]
    pub countries: Option<Vec<String>>,
    #[serde(default, rename = "folder_id")]
    pub folder_id: Option<String>,
    #[serde(default, rename = "keyword_count")]
    pub keyword_count: Option<i64>,
    #[serde(default, rename = "keyword_list_id")]
    pub keyword_list_id: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "owned_by")]
    pub owned_by: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordListsResponse {
    #[serde(default, rename = "keyword_lists")]
    pub keyword_lists: Option<Vec<KeywordListsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordList {
    #[serde(default, rename = "access")]
    pub access: Option<String>,
    #[serde(default, rename = "countries")]
    pub countries: Option<Vec<String>>,
    #[serde(default, rename = "folder_id")]
    pub folder_id: Option<String>,
    #[serde(default, rename = "keyword_count")]
    pub keyword_count: Option<i64>,
    #[serde(default, rename = "keyword_list_id")]
    pub keyword_list_id: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "owned_by")]
    pub owned_by: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordListsPostResponse {
    #[serde(default, rename = "keyword_list")]
    pub keyword_list: Option<KeywordList>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Location {
    #[serde(default, rename = "country_code")]
    pub country_code: Option<String>,
    #[serde(default, rename = "languages")]
    pub languages: Option<Vec<Vec<Value>>>,
    #[serde(default, rename = "locations")]
    pub locations: Option<Vec<Vec<Value>>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LocationsResponse {
    #[serde(default, rename = "location")]
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsItem2 {
    #[serde(default, rename = "mode")]
    pub mode: Option<String>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectCompetitorsResponse {
    #[serde(default, rename = "competitors")]
    pub competitors: Option<Vec<CompetitorsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CompetitorsItem3 {
    #[serde(default, rename = "mode")]
    pub mode: Option<String>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectCompetitorsDeleteResponse {
    #[serde(default, rename = "competitors")]
    pub competitors: Option<Vec<CompetitorsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem3 {
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "language_code")]
    pub language_code: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "location_id")]
    pub location_id: Option<i64>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectKeywordsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem4 {
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "language_code")]
    pub language_code: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "location_id")]
    pub location_id: Option<i64>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectKeywordsDeleteResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem4>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem5 {
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "language_code")]
    pub language_code: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "location_id")]
    pub location_id: Option<i64>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectKeywordsTagsResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem5>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct KeywordsItem6 {
    #[serde(default, rename = "keyword")]
    pub keyword: Option<String>,
    #[serde(default, rename = "language")]
    pub language: Option<String>,
    #[serde(default, rename = "language_code")]
    pub language_code: Option<String>,
    #[serde(default, rename = "location")]
    pub location: Option<String>,
    #[serde(default, rename = "location_id")]
    pub location_id: Option<i64>,
    #[serde(default, rename = "tags")]
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectKeywordsTagsDeleteResponse {
    #[serde(default, rename = "keywords")]
    pub keywords: Option<Vec<KeywordsItem6>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Folder {
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectsItem {
    #[serde(default, rename = "access")]
    pub access: Option<String>,
    #[serde(default, rename = "folder")]
    pub folder: Option<Folder>,
    #[serde(default, rename = "keyword_count")]
    pub keyword_count: Option<i64>,
    #[serde(default, rename = "mode")]
    pub mode: Option<String>,
    #[serde(default, rename = "owned_by")]
    pub owned_by: Option<String>,
    #[serde(default, rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(default, rename = "project_name")]
    pub project_name: Option<String>,
    #[serde(default, rename = "protocol")]
    pub protocol: Option<String>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
    #[serde(default, rename = "verified")]
    pub verified: Option<bool>,
    #[serde(default, rename = "web_analytics_data_key")]
    pub web_analytics_data_key: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProjectsResponse {
    #[serde(default, rename = "projects")]
    pub projects: Option<Vec<ProjectsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Folder2 {
    #[serde(default, rename = "id")]
    pub id: Option<i64>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Project {
    #[serde(default, rename = "access")]
    pub access: Option<String>,
    #[serde(default, rename = "folder")]
    pub folder: Option<Folder2>,
    #[serde(default, rename = "mode")]
    pub mode: Option<String>,
    #[serde(default, rename = "project_id")]
    pub project_id: Option<String>,
    #[serde(default, rename = "project_name")]
    pub project_name: Option<String>,
    #[serde(default, rename = "protocol")]
    pub protocol: Option<String>,
    #[serde(default, rename = "url")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateProjectResponse {
    #[serde(default, rename = "project")]
    pub project: Option<Project>,
}
