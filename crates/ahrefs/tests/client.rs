use ahrefs::{Client, Error, Filter, Params};
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// A client that talks to the mock server.
async fn client_for(server: &MockServer) -> Client {
    Client::builder()
        .base_url(server.uri())
        .expect("mock server uri is a valid url")
        .token("secret")
        .max_retries(1)
        .retry_delay(std::time::Duration::from_millis(10))
        .build()
        .expect("client builds")
}

#[tokio::test]
async fn sends_the_bearer_token() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/site-explorer/domain-rating"))
        .and(header("authorization", "Bearer secret"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "domain_rating": { "domain_rating": 42.0, "ahrefs_rank": 1234 }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let mut params = Params::new();
    params
        .set("target", "example.com")
        .set("date", "2026-11-01");

    let value = client
        .get_value("site-explorer/domain-rating", &params)
        .await
        .expect("request succeeds");

    assert_eq!(value["domain_rating"]["domain_rating"], json!(42.0));
}

#[tokio::test]
async fn sends_parameters_in_the_query_string() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/site-explorer/all-backlinks"))
        .and(query_param("target", "example.com"))
        .and(query_param("select", "url_from,url_to"))
        .and(query_param("limit", "10"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "backlinks": [] })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let mut params = Params::new();
    params.set("target", "example.com");
    params.set("select", "url_from,url_to");
    params.set("limit", 10);

    client
        .get_value("site-explorer/all-backlinks", &params)
        .await
        .expect("request succeeds");
}

#[tokio::test]
async fn posts_a_json_body() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/site-explorer/traffic-metrics"))
        .and(header("content-type", "application/json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "metrics": {} })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let mut body = Params::new();
    body.set("target", "example.com").set("select", "date");

    client
        .post_value("site-explorer/traffic-metrics", &body)
        .await
        .expect("request succeeds");
}

#[tokio::test]
async fn reports_api_errors() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(400).set_body_json(json!({ "error": "target is required" })),
        )
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let params = Params::new();

    let err = client
        .get_value("site-explorer/domain-rating", &params)
        .await
        .expect_err("the request fails");

    match err {
        Error::Api { status, message } => {
            assert_eq!(status, 400);
            assert_eq!(message, "target is required");
        }
        other => panic!("unexpected error: {other:?}"),
    }
}

#[tokio::test]
async fn reports_a_rejected_key() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(403).set_body_json(json!({ "error": "Forbidden" })))
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let params = Params::new();

    let err = client
        .get_value("site-explorer/domain-rating", &params)
        .await
        .expect_err("the request fails");

    assert_eq!(err.status(), Some(403));
    assert_eq!(err.kind(), "unauthorized");
}

#[tokio::test]
async fn retries_a_rate_limited_request() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(429).set_body_json(json!({ "error": "too many" })))
        .up_to_n_times(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ips": [] })))
        .expect(1)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let params = Params::new();

    let value = client
        .get_value("public/crawler-ips", &params)
        .await
        .expect("the retry succeeds");

    assert_eq!(value["ips"], json!([]));
}

#[tokio::test]
async fn gives_up_after_the_last_retry() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(ResponseTemplate::new(429).set_body_json(json!({ "error": "too many" })))
        .expect(2)
        .mount(&server)
        .await;

    let client = client_for(&server).await;
    let params = Params::new();

    let err = client
        .get_value("public/crawler-ips", &params)
        .await
        .expect_err("the request fails");

    assert_eq!(err.kind(), "rate_limited");
}

#[tokio::test]
async fn keeps_the_base_url_path() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v3/public/crawler-ips"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "ips": [] })))
        .expect(1)
        .mount(&server)
        .await;

    let client = Client::builder()
        .base_url(format!("{}/v3", server.uri()))
        .expect("url is valid")
        .build()
        .expect("client builds");

    client
        .get_value("public/crawler-ips", &Params::new())
        .await
        .expect("request succeeds");
}

#[test]
fn filters_render_as_api_expressions() {
    let filter = Filter::and([
        Filter::expr("domain_rating", ahrefs::Condition::Gte(json!(50))),
        Filter::expr("url_from", ahrefs::Condition::Prefix(json!("https"))),
    ]);

    assert_eq!(
        filter.to_json(),
        json!({
            "and": [
                { "field": "domain_rating", "is": ["gte", 50] },
                { "field": "url_from", "is": ["prefix", "https"] }
            ]
        })
    );
}

#[test]
fn nested_filters_render_as_api_expressions() {
    let filter = !Filter::or([
        Filter::expr("anchor", ahrefs::Condition::IsNull),
        Filter::list_any(
            "languages",
            Filter::expr("code", ahrefs::Condition::Eq(json!("en"))),
        ),
    ]);

    assert_eq!(
        filter.to_json(),
        json!({
            "not": {
                "or": [
                    { "field": "anchor", "is": "is_null" },
                    {
                        "field": "languages",
                        "list_is": {
                            "any": { "field": "code", "is": ["eq", "en"] }
                        }
                    }
                ]
            }
        })
    );
}

#[test]
fn parameters_keep_their_order() {
    let mut params = Params::new();
    params.set("target", "example.com");
    params.set("select", "url_from");
    params.set("target", "other.com");

    let pairs = params.to_query_pairs();
    assert_eq!(pairs[0], ("target".to_string(), "other.com".to_string()));
    assert_eq!(pairs[1].0, "select");
    assert_eq!(params.get("select"), Some(&json!("url_from")));
}

#[test]
fn parameters_become_a_json_body() {
    let mut params = Params::new();
    params.set("target", "example.com").set("limit", 10);

    assert_eq!(
        params.to_json(),
        json!({ "target": "example.com", "limit": 10 })
    );
}
