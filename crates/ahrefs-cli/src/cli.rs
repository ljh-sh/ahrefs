use clap::{Parser, Subcommand, ValueEnum};

/// Output rendering.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Format {
    /// JSON object with `ok` and `data`.
    Json,
    /// Human readable table.
    Table,
    /// The API response body, unchanged.
    Raw,
}

/// How the target is matched.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Mode {
    /// The exact URL.
    Exact,
    /// Everything below the path.
    Prefix,
    /// The domain and every path on it.
    Domain,
    /// The domain and its subdomains.
    Subdomains,
}

/// The protocol of the target.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Protocol {
    /// Both HTTP and HTTPS.
    Both,
    /// HTTP only.
    Http,
    /// HTTPS only.
    Https,
}

/// How backlinks are grouped.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Aggregation {
    /// Group links that point at the same target.
    SimilarLinks,
    /// Keep one link per referring domain.
    OnePerDomain,
    /// Keep every link.
    All,
}

/// Which ranking positions to keep.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum TargetPosition {
    /// Positions 1 to 10.
    InTop10,
    /// Positions 1 to 100.
    InTop100,
}

/// The HTTP method used by `ahrefs raw`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Method {
    /// Query parameters travel in the URL.
    Get,
    /// Parameters travel in a JSON body.
    Post,
}

/// Command line interface of the Ahrefs API client.
#[derive(Parser, Debug)]
#[command(
    name = "ahrefs",
    version,
    about = "Command line client for the Ahrefs API v3",
    long_about = None,
    disable_help_subcommand = true
)]
pub struct Cli {
    /// Options that apply to every command.
    #[command(flatten)]
    pub global: GlobalArgs,

    /// The command to run.
    #[command(subcommand)]
    pub command: Command,
}

/// Options that apply to every command.
#[derive(Parser, Debug)]
pub struct GlobalArgs {
    /// Ahrefs API key.
    #[arg(long, global = true, env = "AHREFS_TOKEN", hide_env_values = true)]
    pub token: Option<String>,

    /// API root to talk to.
    #[arg(long, global = true, env = "AHREFS_BASE_URL", default_value = ahrefs::DEFAULT_BASE_URL)]
    pub base_url: String,

    /// How to render the response.
    #[arg(long, global = true, value_enum, default_value_t = Format::Json)]
    pub format: Format,

    /// How often a rate limited request is retried.
    #[arg(long, global = true, default_value_t = 3)]
    pub retries: u32,

    /// Request timeout in seconds.
    #[arg(long, global = true, default_value_t = 300)]
    pub timeout: u32,
}

/// The top level commands.
#[derive(Subcommand, Debug)]
pub enum Command {
    /// Site Explorer reports.
    #[command(subcommand)]
    SiteExplorer(SiteExplorerCommand),

    /// Keywords Explorer reports.
    #[command(subcommand)]
    KeywordsExplorer(KeywordsExplorerCommand),

    /// Public endpoints, such as the crawler IP ranges.
    #[command(subcommand)]
    Public(PublicCommand),

    /// Call any endpoint directly.
    Raw {
        /// Endpoint path, such as `site-explorer/domain-rating`.
        path: String,

        /// HTTP method.
        #[arg(long, value_enum, default_value_t = Method::Get)]
        method: Method,

        /// Parameter as `key=value`, repeatable.
        #[arg(long = "param", value_name = "KEY=VALUE")]
        params: Vec<String>,
    },
}

/// Site Explorer commands.
#[derive(Subcommand, Debug)]
pub enum SiteExplorerCommand {
    /// Domain rating of a target.
    DomainRating {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Date to report metrics on.
        #[arg(long, default_value_t = today())]
        date: String,
    },

    /// Domain rating over time.
    DomainRatingHistory {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
    },

    /// Traffic and keyword metrics of a target.
    Metrics {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Date to report metrics on.
        #[arg(long, default_value_t = today())]
        date: String,
    },

    /// Backlink statistics of a target.
    BacklinksStats {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Date to report metrics on.
        #[arg(long, default_value_t = today())]
        date: String,
    },

    /// Outlink statistics of a target.
    OutlinksStats {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Date to report metrics on.
        #[arg(long, default_value_t = today())]
        date: String,
    },

    /// Backlinks pointing at a target.
    Backlinks {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Filter expression.
        #[arg(long = "where")]
        filter: Option<String>,
        /// Column to sort by.
        #[arg(long)]
        order_by: Option<String>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// How the target is matched.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// The protocol of the target.
        #[arg(long, value_enum)]
        protocol: Option<Protocol>,
        /// How backlinks are grouped.
        #[arg(long, value_enum)]
        aggregation: Option<Aggregation>,
        /// Time frame that adds lost backlinks.
        #[arg(long)]
        history: Option<String>,
    },

    /// Broken backlinks pointing at a target.
    BrokenBacklinks {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Filter expression.
        #[arg(long = "where")]
        filter: Option<String>,
        /// Column to sort by.
        #[arg(long)]
        order_by: Option<String>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// How the target is matched.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// The protocol of the target.
        #[arg(long, value_enum)]
        protocol: Option<Protocol>,
    },

    /// Domains linking to a target.
    Refdomains {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Filter expression.
        #[arg(long = "where")]
        filter: Option<String>,
        /// Column to sort by.
        #[arg(long)]
        order_by: Option<String>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// How the target is matched.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// The protocol of the target.
        #[arg(long, value_enum)]
        protocol: Option<Protocol>,
    },

    /// Anchors of the backlinks pointing at a target.
    Anchors {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Filter expression.
        #[arg(long = "where")]
        filter: Option<String>,
        /// Column to sort by.
        #[arg(long)]
        order_by: Option<String>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// How the target is matched.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// The protocol of the target.
        #[arg(long, value_enum)]
        protocol: Option<Protocol>,
    },

    /// Organic keywords a target ranks for.
    OrganicKeywords {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Filter expression.
        #[arg(long = "where")]
        filter: Option<String>,
        /// Column to sort by.
        #[arg(long)]
        order_by: Option<String>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// How the target is matched.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// The protocol of the target.
        #[arg(long, value_enum)]
        protocol: Option<Protocol>,
        /// Country code to report on.
        #[arg(long)]
        country: Option<String>,
    },

    /// Pages of a target that bring in the most traffic.
    TopPages {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Filter expression.
        #[arg(long = "where")]
        filter: Option<String>,
        /// Column to sort by.
        #[arg(long)]
        order_by: Option<String>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// How the target is matched.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// The protocol of the target.
        #[arg(long, value_enum)]
        protocol: Option<Protocol>,
        /// Country code to report on.
        #[arg(long)]
        country: Option<String>,
        /// First day of the reporting period.
        #[arg(long)]
        date_from: Option<String>,
        /// Last day of the reporting period.
        #[arg(long)]
        date_to: Option<String>,
    },
}

/// Keywords Explorer commands.
#[derive(Subcommand, Debug)]
pub enum KeywordsExplorerCommand {
    /// Metrics of one or more keywords.
    Overview {
        /// Keywords to report on, comma separated.
        #[arg(long, required_unless_present = "keyword_list_id")]
        keywords: Option<String>,
        /// Keyword list to report on.
        #[arg(long)]
        keyword_list_id: Option<u64>,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Filter expression.
        #[arg(long = "where")]
        filter: Option<String>,
        /// Column to sort by.
        #[arg(long)]
        order_by: Option<String>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// Country code, or `all`.
        #[arg(long)]
        country: Option<String>,
        /// Domain or URL whose rankings narrow the report down.
        #[arg(long)]
        target: Option<String>,
        /// How the target is matched.
        #[arg(long, value_enum)]
        mode: Option<Mode>,
        /// Which ranking positions to keep.
        #[arg(long, value_enum)]
        target_position: Option<TargetPosition>,
    },

    /// Monthly search volume over time.
    VolumeHistory {
        /// Keywords to report on, comma separated.
        #[arg(long)]
        keywords: Option<String>,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// Country code, or `all`.
        #[arg(long)]
        country: Option<String>,
    },

    /// Keywords that match the given keywords.
    MatchingTerms {
        /// Keywords to expand, comma separated.
        #[arg(long)]
        keywords: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// Country code, or `all`.
        #[arg(long)]
        country: Option<String>,
    },

    /// Keywords related to the given keywords.
    RelatedTerms {
        /// Keywords to expand, comma separated.
        #[arg(long)]
        keywords: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// Country code, or `all`.
        #[arg(long)]
        country: Option<String>,
    },

    /// Search suggestions for the given keywords.
    SearchSuggestions {
        /// Keywords to expand, comma separated.
        #[arg(long)]
        keywords: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
        /// Country code, or `all`.
        #[arg(long)]
        country: Option<String>,
    },

    /// Search volume split by country.
    VolumeByCountry {
        /// Keywords to report on, comma separated.
        #[arg(long)]
        keywords: String,
        /// Columns to return.
        #[arg(long, value_delimiter = ',')]
        select: Option<Vec<String>>,
        /// Maximum number of rows.
        #[arg(long)]
        limit: Option<u32>,
    },
}

/// Public commands.
#[derive(Subcommand, Debug)]
pub enum PublicCommand {
    /// IP addresses Ahrefs crawlers run from.
    CrawlerIps,

    /// IP ranges Ahrefs crawlers run from.
    CrawlerIpRanges,

    /// Domain rating of a target, without spending API units.
    DomainRatingFree {
        /// Domain or URL to report on.
        #[arg(long)]
        target: String,
    },

    /// Domains with the highest domain rating.
    DomainRatingTopDomains,
}

/// Today in UTC, as `YYYY-MM-DD`.
pub fn today() -> String {
    let seconds = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or_default();
    let days = seconds.div_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    format!("{year:04}-{month:02}-{day:02}")
}

/// Turns a day count since the Unix epoch into a civil date.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn civil_dates_are_correct() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(19_723), (2024, 1, 1));
        assert_eq!(civil_from_days(19_826), (2024, 4, 13));
        assert_eq!(civil_from_days(20_775), (2026, 11, 18));
    }
}
