//! Response models for the `public` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PrefixesItem {
    #[serde(default, rename = "ipv4Prefix")]
    pub ipv4_prefix: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CrawlerIpRangesResponse {
    #[serde(default, rename = "prefixes")]
    pub prefixes: Option<Vec<PrefixesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IpsItem {
    #[serde(default, rename = "ip_address")]
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CrawlerIpsResponse {
    #[serde(default, rename = "ips")]
    pub ips: Option<Vec<IpsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRating {
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "license")]
    pub license: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRatingFreeResponse {
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<DomainRating>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TargetsItem {
    #[serde(default, rename = "target")]
    pub target: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRating2 {
    #[serde(default, rename = "license")]
    pub license: Option<String>,
    #[serde(default, rename = "targets")]
    pub targets: Option<Vec<TargetsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRatingFreePostResponse {
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<DomainRating2>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainsItem {
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "domain_rating")]
    pub domain_rating: Option<f64>,
    #[serde(default, rename = "rank")]
    pub rank: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DomainRatingTopDomainsResponse {
    #[serde(default, rename = "domains")]
    pub domains: Option<Vec<DomainsItem>>,
}
