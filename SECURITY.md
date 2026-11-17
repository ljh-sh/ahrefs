# Security Policy

## Supported versions

Only the latest released version is supported.

## Reporting a vulnerability

Please report vulnerabilities privately through GitHub's
[security advisory form](https://github.com/ljh-sh/ahrefs/security/advisories/new)
instead of opening a public issue.

You can expect an acknowledgement within a week. If the report is accepted, a
fix is released as soon as it is ready, and the advisory is published with
credit to the reporter unless they prefer to stay anonymous.

## Handling of API keys

The client never writes an API key to disk. It reads the key from
`AHREFS_TOKEN` or from the `--token` flag, sends it as a bearer token, and
redacts it from `--help` output.

Use a key with a usage limit and an expiration date. Keys are created in
Ahrefs under Account settings / API keys.
