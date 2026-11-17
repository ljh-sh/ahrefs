//! Response models for the `web-analytics` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "browser_version")]
    pub browser_version: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrowserVersionsResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "browser_version")]
    pub browser_version: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrowserVersionsChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem2 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "browser")]
    pub browser: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrowsersResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem2 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "browser")]
    pub browser: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BrowsersChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem3 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "avg_session_length")]
    pub avg_session_length: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem3 {
    #[serde(default, rename = "city")]
    pub city: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CitiesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem3>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem4 {
    #[serde(default, rename = "city")]
    pub city: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CitiesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem4>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem4 {
    #[serde(default, rename = "continent")]
    pub continent: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContinentsResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem4>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem5 {
    #[serde(default, rename = "continent")]
    pub continent: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ContinentsChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem5>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem5 {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CountriesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem5>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem6 {
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CountriesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem6>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem6 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "device")]
    pub device: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DevicesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem6>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem7 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "device")]
    pub device: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DevicesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem7>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem7 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "entries")]
    pub entries: Option<i64>,
    #[serde(default, rename = "entry_page")]
    pub entry_page: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EntryPagesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem7>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem8 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "entries")]
    pub entries: Option<i64>,
    #[serde(default, rename = "entry_page")]
    pub entry_page: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EntryPagesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem8>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem8 {
    #[serde(default, rename = "exit_page")]
    pub exit_page: Option<String>,
    #[serde(default, rename = "exit_rate")]
    pub exit_rate: Option<f64>,
    #[serde(default, rename = "exits")]
    pub exits: Option<i64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExitPagesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem8>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem9 {
    #[serde(default, rename = "exit_page")]
    pub exit_page: Option<String>,
    #[serde(default, rename = "exit_rate")]
    pub exit_rate: Option<f64>,
    #[serde(default, rename = "exits")]
    pub exits: Option<i64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExitPagesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem9>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem9 {
    #[serde(default, rename = "browser_language")]
    pub browser_language: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LanguagesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem9>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem10 {
    #[serde(default, rename = "browser_language")]
    pub browser_language: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LanguagesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem10>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem10 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "os")]
    pub os: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystemsResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem10>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem11 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "os")]
    pub os: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystemsChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem11>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem11 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "os_version")]
    pub os_version: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystemsVersionsResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem11>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem12 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "os_version")]
    pub os_version: Option<String>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OperatingSystemsVersionsChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem12>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem12 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "source_referer")]
    pub source_referer: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferrersResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem12>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem13 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "source_referer")]
    pub source_referer: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferrersChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem13>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem13 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "source_channel")]
    pub source_channel: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourceChannelsResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem13>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem14 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "source_channel")]
    pub source_channel: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourceChannelsChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem14>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem14 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourcesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem14>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem15 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "source")]
    pub source: Option<String>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SourcesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem15>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Stats {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "avg_session_length")]
    pub avg_session_length: Option<f64>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
    #[serde(default, rename = "visits")]
    pub visits: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Stats>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem15 {
    #[serde(default, rename = "avg_page_visit_duration_sec")]
    pub avg_page_visit_duration_sec: Option<i64>,
    #[serde(default, rename = "page")]
    pub page: Option<String>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopPagesResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem15>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem16 {
    #[serde(default, rename = "avg_page_visit_duration_sec")]
    pub avg_page_visit_duration_sec: Option<i64>,
    #[serde(default, rename = "page")]
    pub page: Option<String>,
    #[serde(default, rename = "pageviews")]
    pub pageviews: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TopPagesChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem16>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatsItem16 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "utm_param")]
    pub utm_param: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UtmParamsResponse {
    #[serde(default, rename = "stats")]
    pub stats: Option<Vec<StatsItem16>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointsItem17 {
    #[serde(default, rename = "avg_session_duration_sec")]
    pub avg_session_duration_sec: Option<i64>,
    #[serde(default, rename = "session_bounce_rate")]
    pub session_bounce_rate: Option<f64>,
    #[serde(default, rename = "timestamp")]
    pub timestamp: Option<String>,
    #[serde(default, rename = "utm_param")]
    pub utm_param: Option<String>,
    #[serde(default, rename = "visitors")]
    pub visitors: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UtmParamsChartResponse {
    #[serde(default, rename = "points")]
    pub points: Option<Vec<PointsItem17>>,
}
