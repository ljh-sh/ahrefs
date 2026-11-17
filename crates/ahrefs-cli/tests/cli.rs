use std::process::Command;

use serde_json::Value;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// Runs the CLI against the mock server and returns stdout, stderr and the
/// exit code.
fn run(args: &[&str], base_url: &str) -> (String, String, bool) {
    let output = Command::new(env!("CARGO_BIN_EXE_ahrefs"))
        .args(args)
        .args(["--base-url", base_url])
        .output()
        .expect("the binary runs");

    (
        String::from_utf8_lossy(&output.stdout).into_owned(),
        String::from_utf8_lossy(&output.stderr).into_owned(),
        output.status.success(),
    )
}

/// Parses a JSON line.
fn json(text: &str) -> Value {
    serde_json::from_str(text.trim()).expect("output is JSON")
}

#[tokio::test]
async fn prints_a_json_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/public/crawler-ips"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"ips": [{"ip_address": "1.2.3.4"}]})),
        )
        .mount(&server)
        .await;

    let (stdout, stderr, ok) = run(&["public", "crawler-ips"], &server.uri());

    assert!(ok, "the command succeeds: {stderr}");
    assert!(stderr.is_empty(), "nothing goes to stderr: {stderr}");

    let value = json(&stdout);
    assert_eq!(value["ok"], serde_json::json!(true));
    assert_eq!(
        value["data"]["ips"][0]["ip_address"],
        serde_json::json!("1.2.3.4")
    );
}

#[tokio::test]
async fn prints_a_table() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/public/crawler-ips"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"ips": [{"ip_address": "1.2.3.4"}]})),
        )
        .mount(&server)
        .await;

    let (stdout, _, ok) = run(
        &["--format", "table", "public", "crawler-ips"],
        &server.uri(),
    );

    assert!(ok);
    assert_eq!(stdout.trim(), "ip_address\n1.2.3.4");
}

#[tokio::test]
async fn reports_errors_on_stderr() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(serde_json::json!({"error": "target is required"})),
        )
        .mount(&server)
        .await;

    let (stdout, stderr, ok) = run(&["public", "crawler-ips"], &server.uri());

    assert!(!ok, "the command fails");
    assert!(stdout.is_empty(), "nothing goes to stdout: {stdout}");

    let value = json(&stderr);
    assert_eq!(value["ok"], serde_json::json!(false));
    assert_eq!(value["error"]["kind"], serde_json::json!("api_error"));
    assert_eq!(value["error"]["status"], serde_json::json!(400));
}

#[tokio::test]
async fn sends_the_default_columns() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/site-explorer/all-backlinks"))
        .and(query_param("target", "example.com"))
        .and(query_param(
            "select",
            "url_from,url_to,anchor,domain_rating_source,first_seen,last_seen,is_dofollow,is_lost",
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"backlinks": []})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let (stdout, stderr, ok) = run(
        &["site-explorer", "backlinks", "--target", "example.com"],
        &server.uri(),
    );

    assert!(ok, "the command succeeds: {stderr}");
    assert_eq!(json(&stdout)["ok"], serde_json::json!(true));
}

#[tokio::test]
async fn passes_filter_shorthand_to_the_api() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/site-explorer/all-backlinks"))
        .and(query_param(
            "where",
            r#"{"field":"domain_rating","is":["gte",50]}"#,
        ))
        .respond_with(
            ResponseTemplate::new(200).set_body_json(serde_json::json!({"backlinks": []})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let (stdout, stderr, ok) = run(
        &[
            "site-explorer",
            "backlinks",
            "--target",
            "example.com",
            "--where",
            "domain_rating>=50",
        ],
        &server.uri(),
    );

    assert!(ok, "the command succeeds: {stderr}");
    assert_eq!(json(&stdout)["ok"], serde_json::json!(true));
}

#[tokio::test]
async fn calls_any_endpoint_through_raw() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/site-explorer/domain-rating"))
        .and(query_param("target", "example.com"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!({"domain_rating": {"domain_rating": 7.0}})),
        )
        .expect(1)
        .mount(&server)
        .await;

    let (stdout, stderr, ok) = run(
        &[
            "raw",
            "site-explorer/domain-rating",
            "--param",
            "target=example.com",
        ],
        &server.uri(),
    );

    assert!(ok, "the command succeeds: {stderr}");
    assert_eq!(
        json(&stdout)["data"]["domain_rating"]["domain_rating"],
        serde_json::json!(7.0)
    );
}

#[tokio::test]
async fn rejects_a_broken_filter() {
    let server = MockServer::start().await;

    let (stdout, stderr, ok) = run(
        &[
            "site-explorer",
            "backlinks",
            "--target",
            "example.com",
            "--where",
            "nonsense",
        ],
        &server.uri(),
    );

    assert!(!ok, "the command fails");
    assert!(stdout.is_empty());
    assert_eq!(
        json(&stderr)["error"]["kind"],
        serde_json::json!("invalid_param")
    );
}
