# Contributing

Thanks for taking the time to help.

## Development

```sh
cargo build
cargo test
cargo fmt --all --check
cargo clippy --all-targets -- -D warnings
```

Tests do not need an API key. They run against a local mock server.

## Regenerating the response models

The response models in `crates/ahrefs/src/model/` are generated from
`openapi/ahrefs-v3.json`, which is the OpenAPI specification published at
<https://docs.ahrefs.com/openapi.json>.

```sh
python3 tools/generate.py
```

Commit the regenerated files together with the specification update. Never
edit the generated files by hand.

## Pull requests

- Keep a change focused on one topic.
- Add a test for every behaviour change.
- Update `README.md` and `docs/` when the command line changes.
- Add a line to `CHANGELOG.md` under `Unreleased`.
