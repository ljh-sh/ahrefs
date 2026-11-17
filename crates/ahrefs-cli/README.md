# ahrefs-cli

The `ahrefs` command line client for the [Ahrefs API v3](https://docs.ahrefs.com/docs/api/).

```sh
cargo install ahrefs-cli --locked

export AHREFS_TOKEN="your-api-key"
ahrefs site-explorer domain-rating --target ahrefs.com
ahrefs --format table site-explorer backlinks --target ahrefs.com --limit 10
```

Output is compact JSON with an `ok` field. Errors go to stderr with a non-zero
exit code.

License: MIT
