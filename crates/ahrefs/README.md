# ahrefs

A Rust client for the [Ahrefs API v3](https://docs.ahrefs.com/docs/api/).

```rust,no_run
use ahrefs::{endpoints::site_explorer, Client};

# async fn run() -> ahrefs::Result<()> {
let client = Client::from_env()?;
let rating = site_explorer::domain_rating(&client, "ahrefs.com", "2026-11-01").await?;
println!("{:?}", rating.domain_rating);
# Ok(())
# }
```

- Bearer authentication, read from `AHREFS_TOKEN` or set on the builder.
- Retries on `429 Too Many Requests` and on server errors.
- Response models for every endpoint, generated from the official OpenAPI
  specification.
- A typed builder for the `where` filter language.

Documentation: <https://docs.rs/ahrefs>

License: MIT
