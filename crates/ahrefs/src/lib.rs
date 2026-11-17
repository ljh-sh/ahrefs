//! A Rust client for the [Ahrefs API v3](https://docs.ahrefs.com/docs/api/).
//!
//! The client covers the whole API surface. Endpoints that are commonly used
//! have typed wrappers under [`endpoints`]; anything else can be reached with
//! [`Client::get_value`] and a [`Params`] set.
//!
//! ```no_run
//! use ahrefs::{endpoints::site_explorer, Client};
//!
//! # async fn run() -> ahrefs::Result<()> {
//! let client = Client::from_env()?;
//! let rating = site_explorer::domain_rating(&client, "example.com", "2026-11-01").await?;
//! println!("{:?}", rating.domain_rating);
//! # Ok(())
//! # }
//! ```
//!
//! Every response model has optional fields: the API only returns the columns
//! asked for through the `select` parameter, so a response is always a subset
//! of the documented schema.

#![warn(missing_docs)]

/// The HTTP client and its builder.
pub mod client;
pub mod endpoints;
/// Error types.
pub mod error;
pub mod filter;
pub mod model;
/// Request parameters and their shared enums.
pub mod params;

pub use client::{Client, ClientBuilder, RawResponse, DEFAULT_BASE_URL, TOKEN_ENV};
pub use error::{Error, Result};
pub use filter::{Condition, Expr, Filter, ListCondition};
pub use params::{Aggregation, Mode, Output, Params, Protocol, Report, TargetPosition};
