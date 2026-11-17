# API and library guide

The `ahrefs` crate is a client for the [Ahrefs API v3](https://docs.ahrefs.com/docs/api/).
The base URL is `https://api.ahrefs.com/v3`, the API key travels as a bearer
token, and the rate limit is 60 requests per minute by default.

## Client

```rust,no_run
use ahrefs::Client;

# async fn run() -> ahrefs::Result<()> {
let client = Client::from_env()?;
# Ok(())
# }
```

`Client::from_env` reads `AHREFS_TOKEN`. A client without a key still works
for the endpoints that do not need one.

```rust,no_run
use ahrefs::Client;

# fn run() -> ahrefs::Result<()> {
let client = Client::builder()
    .base_url("https://api.ahrefs.com/v3")?
    .token("your-api-key")
    .max_retries(5)
    .timeout(std::time::Duration::from_secs(60))
    .build()?;
# Ok(())
# }
```

A `429` or a server error is retried with an exponential backoff. The
`Retry-After` header wins over the computed delay.

## Parameters

`Params` holds the parameters of one call. The API takes the same names in the
query string of a `GET` and in the JSON body of a `POST`, so one type covers
both.

```rust,no_run
use ahrefs::Params;

# fn run() {
let mut params = Params::new();
params.set("target", "ahrefs.com").set("date", "2026-11-01");
# }
```

## Endpoints

The commonly used endpoints have typed wrappers.

```rust,no_run
use ahrefs::{endpoints::site_explorer, Client, Report};

# async fn run() -> ahrefs::Result<()> {
let client = Client::from_env()?;

let rating = site_explorer::domain_rating(&client, "ahrefs.com", "2026-11-01").await?;

let query = site_explorer::BacklinksQuery {
    target: "ahrefs.com".into(),
    report: Report::select(&["url_from", "url_to", "anchor", "domain_rating_source"]),
    ..Default::default()
};
let backlinks = site_explorer::backlinks(&client, &query).await?;
# Ok(())
# }
```

The groups are `endpoints::site_explorer`, `endpoints::keywords_explorer` and
`endpoints::public`.

## Any endpoint

Everything else goes through `get_value` and `post_value`, which take the
endpoint path and a parameter set.

```rust,no_run
use ahrefs::{Client, Params};

# async fn run() -> ahrefs::Result<()> {
let client = Client::from_env()?;

let mut params = Params::new();
params.set("target", "ahrefs.com");

let value = client.get_value("site-explorer/traffic-metrics", &params).await?;
# Ok(())
# }
```

## Response models

The models under `ahrefs::model` are generated from the OpenAPI specification
that Ahrefs publishes at <https://docs.ahrefs.com/openapi.json>. A copy lives
in `openapi/ahrefs-v3.json` in this repository.

```sh
python3 tools/generate.py
```

Every field is optional. The API only returns the columns asked for through
the `select` parameter, so a response is always a subset of the documented
schema.

## Filters

The `where` parameter takes a small JSON language. `ahrefs::filter` builds it
without hand written JSON.

```rust,no_run
use ahrefs::{Condition, Filter};

# fn run() {
let filter = Filter::and([
    Filter::expr("domain_rating_source", Condition::Gte(50.into())),
    Filter::expr("url_from", Condition::Prefix("https".into())),
]);
println!("{}", filter.to_json());
# }
```

`Filter::and`, `Filter::or` and `Filter::list_any` combine conditions. The
`not` operator is the `Not` trait, so `!filter` negates a filter.

## Errors

Every failure is an `ahrefs::Error`.

| Variant | Meaning |
| --- | --- |
| `MissingToken` | No API key was configured |
| `Unauthorized` | The API key was rejected |
| `Api` | The API refused the request, with the status code |
| `RateLimited` | The request stayed rate limited after every retry |
| `Request` | The API could not be reached |
| `Decode` | The response was not valid JSON |
| `InvalidParam` | A parameter was rejected before the request was sent |

`Error::kind` returns a short stable tag, `Error::status` returns the HTTP
status when there is one.
