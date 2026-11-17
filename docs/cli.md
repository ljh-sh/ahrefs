# CLI reference

The `ahrefs` command talks to the Ahrefs API v3.

```
ahrefs [OPTIONS] <COMMAND>
```

## Global options

| Option | Default | Meaning |
| --- | --- | --- |
| `--token <TOKEN>` | `$AHREFS_TOKEN` | API key, sent as a bearer token |
| `--base-url <URL>` | `https://api.ahrefs.com/v3` | API root |
| `--format <FORMAT>` | `json` | `json`, `table` or `raw` |
| `--retries <N>` | `3` | How often a rate limited request is retried |
| `--timeout <SECONDS>` | `300` | Request timeout |

Global options work before or after the command name.

## Output

`json` prints one line of compact JSON:

```json
{"data":{"domain_rating":{"domain_rating":92.4}},"ok":true}
```

`table` prints a table. A response with a list becomes a table of rows, a
response with a single object becomes a table of key and value.

`raw` prints the API response body unchanged, which is what you want when
piping into another tool.

Errors go to stderr and the exit code is non-zero:

```json
{"error":{"kind":"api_error","message":"api error (status 400): target is required","status":400},"ok":false}
```

| `kind` | Meaning |
| --- | --- |
| `unauthorized` | The API key is missing, expired or rejected |
| `api_error` | The API refused the request |
| `rate_limited` | The request stayed rate limited after every retry |
| `request_failed` | The API could not be reached |
| `decode_failed` | The response was not valid JSON |
| `invalid_param` | A parameter was rejected before the request was sent |

## site-explorer

```
ahrefs site-explorer <COMMAND>
```

| Command | Purpose |
| --- | --- |
| `domain-rating` | Domain rating of a target |
| `domain-rating-history` | Domain rating over time |
| `metrics` | Traffic and keyword metrics of a target |
| `backlinks-stats` | Backlink statistics of a target |
| `outlinks-stats` | Outlink statistics of a target |
| `backlinks` | Backlinks pointing at a target |
| `broken-backlinks` | Broken backlinks pointing at a target |
| `refdomains` | Domains linking to a target |
| `anchors` | Anchors of the backlinks pointing at a target |
| `organic-keywords` | Organic keywords a target ranks for |
| `top-pages` | Pages of a target that bring in the most traffic |

The single number commands take `--target` and `--date`. The date defaults to
today, in UTC.

```sh
ahrefs site-explorer domain-rating --target ahrefs.com
ahrefs site-explorer domain-rating --target ahrefs.com --date 2026-11-01
```

The report commands take `--target` plus the shared report options:

| Option | Meaning |
| --- | --- |
| `--select <COLUMNS>` | Columns to return, comma separated |
| `--where <FILTER>` | Filter expression |
| `--order-by <COLUMN>` | Column to sort by |
| `--limit <N>` | Maximum number of rows |
| `--mode <MODE>` | `exact`, `prefix`, `domain` or `subdomains` |
| `--protocol <PROTOCOL>` | `both`, `http` or `https` |

```sh
ahrefs site-explorer backlinks --target ahrefs.com --limit 20
ahrefs site-explorer refdomains --target ahrefs.com --where "domain_rating>=50"
ahrefs site-explorer top-pages --target ahrefs.com --country us --date-from 2026-10-01
```

`site-explorer backlinks` also takes `--aggregation` (`similar_links`,
`1_per_domain` or `all`) and `--history` (`live`, `since:<date>` or
`all_time`).

## keywords-explorer

```
ahrefs keywords-explorer <COMMAND>
```

| Command | Purpose |
| --- | --- |
| `overview` | Metrics of one or more keywords |
| `volume-history` | Monthly search volume over time |
| `matching-terms` | Keywords that match the given keywords |
| `related-terms` | Keywords related to the given keywords |
| `search-suggestions` | Search suggestions for the given keywords |
| `volume-by-country` | Search volume split by country |

The keyword commands take `--keywords` (comma separated) and `--country`, which
defaults to `us`. `overview` also takes `--target`, `--mode`,
`--target-position` and `--keyword-list-id`.

```sh
ahrefs keywords-explorer overview --keywords "rust cli,seo api"
ahrefs keywords-explorer matching-terms --keywords "rust cli" --country all
```

## public

```
ahrefs public <COMMAND>
```

| Command | Purpose |
| --- | --- |
| `crawler-ips` | IP addresses Ahrefs crawlers run from |
| `crawler-ip-ranges` | IP ranges Ahrefs crawlers run from |
| `domain-rating-free` | Domain rating of a target, without spending API units |
| `domain-rating-top-domains` | Domains with the highest domain rating |

`crawler-ips` and `crawler-ip-ranges` need no API key.

```sh
ahrefs public crawler-ip-ranges
```

## raw

Every endpoint of the API is reachable through `raw`. Parameters are passed as
`key=value` pairs.

```sh
ahrefs raw site-explorer/domain-rating --param target=ahrefs.com --param date=2026-11-01
ahrefs raw site-explorer/traffic-metrics --method post --param target=ahrefs.com
```

## Filters

`--where` accepts the filter language of the API. A JSON object is passed
through unchanged:

```sh
ahrefs site-explorer backlinks --target ahrefs.com \
  --where '{"and":[{"field":"domain_rating_source","is":["gte",50]}]}'
```

A shorthand is available for the common comparisons. `field=value`,
`field!=value`, `field>value`, `field>=value`, `field<value` and
`field<=value` become an equality or comparison check. Numbers and booleans
are recognised, everything else stays a string.

```sh
ahrefs site-explorer backlinks --target ahrefs.com --where "domain_rating_source>=50"
```

## Cost

The API charges by the number of rows and the number of fields in a response,
with a minimum of 50 units per request. Keep `--select` narrow and `--limit`
small while exploring.
