# ahrefs

> Ahrefs API v3 的命令行客户端 + Rust 库。JSON 输出，适合脚本与 AI agent。
>
> A command line client and Rust library for the Ahrefs API v3. JSON output,
> built for shell pipelines and AI agents.

[English](#english) · [安装](#安装) · [快速开始](#快速开始)

## 安装

```sh
cargo install ahrefs-cli --locked
```

或者从源码构建：

```sh
git clone https://github.com/ljh-sh/ahrefs
cd ahrefs
cargo install --path crates/ahrefs-cli --locked
```

## 快速开始

API key 从 `AHREFS_TOKEN` 环境变量读取，也可以在 Ahrefs 后台
（Account settings / API keys）创建。

```sh
export AHREFS_TOKEN="your-api-key"

# 域名的 DR
ahrefs site-explorer domain-rating --target ahrefs.com

# 外链（表格输出）
ahrefs --format table site-explorer backlinks --target ahrefs.com --limit 10

# 关键词指标
ahrefs keywords-explorer overview --keywords "rust cli,seo api"
```

输出是一行紧凑 JSON，成功时 `ok` 为 `true`：

```json
{"data":{"domain_rating":{"domain_rating":92.4}},"ok":true}
```

失败时错误写到 stderr，退出码非 0：

```json
{"error":{"kind":"unauthorized","message":"authentication failed (status 403): Error: Forbidden","status":403},"ok":false}
```

## 给 AI agent 用

```md
Use `ahrefs` for the Ahrefs API v3. Install if missing: `cargo install ahrefs-cli`.
Set AHREFS_TOKEN first. JSON output, check `ok`. Run `ahrefs --help` for
subcommands, `ahrefs site-explorer --help` for Site Explorer reports.
```

## 文档

- [命令行参考](docs/cli.md)
- [API 与库用法](docs/api.md)

## 许可

[MIT](LICENSE)

---

# English

`ahrefs` is a client for the [Ahrefs API v3](https://docs.ahrefs.com/docs/api/).
It ships two crates:

| Crate | What it is |
| --- | --- |
| [`ahrefs`](crates/ahrefs) | The Rust library: typed client, response models, filter builder |
| [`ahrefs-cli`](crates/ahrefs-cli) | The `ahrefs` binary |

The response models are generated from the official OpenAPI specification,
which is vendored in `openapi/ahrefs-v3.json`. Regenerate them with:

```sh
python3 tools/generate.py
```

## Install

```sh
cargo install ahrefs-cli --locked
```

## Use

```sh
export AHREFS_TOKEN="your-api-key"

ahrefs site-explorer domain-rating --target ahrefs.com
ahrefs --format table site-explorer backlinks --target ahrefs.com --limit 10
ahrefs keywords-explorer overview --keywords "rust cli,seo api"
ahrefs public crawler-ips
```

`site-explorer`, `keywords-explorer` and `public` cover the reports people use
most. Every other endpoint is reachable through `raw`:

```sh
ahrefs raw site-explorer/traffic-metrics --method post --param target=ahrefs.com
```

## Library

```rust,no_run
use ahrefs::{endpoints::site_explorer, Client};

# async fn run() -> ahrefs::Result<()> {
let client = Client::from_env()?;
let rating = site_explorer::domain_rating(&client, "ahrefs.com", "2026-11-01").await?;
println!("{:?}", rating.domain_rating);
# Ok(())
# }
```

## Docs

- [CLI reference](docs/cli.md)
- [API and library guide](docs/api.md)

## License

[MIT](LICENSE)
