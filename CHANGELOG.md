# Changelog

All notable changes to this project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-11-18

### Added

- `ahrefs` crate: a typed client for the Ahrefs API v3 with bearer
  authentication, retry on rate limits and server errors, and response models
  generated from the official OpenAPI specification.
- `ahrefs-cli` crate: the `ahrefs` command line client, with `site-explorer`,
  `keywords-explorer` and `public` command groups plus a `raw` escape hatch
  for every other endpoint.
- JSON, table and raw output formats.
- `tools/generate.py`, which regenerates the response models from
  `openapi/ahrefs-v3.json`.

[Unreleased]: https://github.com/ljh-sh/ahrefs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/ljh-sh/ahrefs/releases/tag/v0.1.0
