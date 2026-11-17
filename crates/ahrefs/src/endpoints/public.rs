//! Public endpoints. These need no API key.

use crate::client::Client;
use crate::model::public::{
    CrawlerIpRangesResponse, CrawlerIpsResponse, DomainRatingFreeResponse,
    DomainRatingTopDomainsResponse,
};
use crate::params::Params;
use crate::Result;

/// The IP addresses Ahrefs crawlers run from.
pub async fn crawler_ips(client: &Client) -> Result<CrawlerIpsResponse> {
    let params = Params::new();
    client.get("public/crawler-ips", &params).await
}

/// The IP ranges Ahrefs crawlers run from.
pub async fn crawler_ip_ranges(client: &Client) -> Result<CrawlerIpRangesResponse> {
    let params = Params::new();
    client.get("public/crawler-ip-ranges", &params).await
}

/// The domain rating of a target, without spending API units.
pub async fn domain_rating_free(client: &Client, target: &str) -> Result<DomainRatingFreeResponse> {
    let mut params = Params::new();
    params.set("target", target);
    client.get("public/domain-rating-free", &params).await
}

/// The domains with the highest domain rating.
pub async fn domain_rating_top_domains(client: &Client) -> Result<DomainRatingTopDomainsResponse> {
    let params = Params::new();
    client
        .get("public/domain-rating-top-domains", &params)
        .await
}
