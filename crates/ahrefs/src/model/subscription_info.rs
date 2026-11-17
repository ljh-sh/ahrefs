//! Response models for the `subscription-info` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitsAndUsage {
    #[serde(default, rename = "api_key_expiration_date")]
    pub api_key_expiration_date: Option<String>,
    #[serde(default, rename = "subscription")]
    pub subscription: Option<String>,
    #[serde(default, rename = "units_limit_api_key")]
    pub units_limit_api_key: Option<i64>,
    #[serde(default, rename = "units_limit_workspace")]
    pub units_limit_workspace: Option<i64>,
    #[serde(default, rename = "units_usage_api_key")]
    pub units_usage_api_key: Option<i64>,
    #[serde(default, rename = "units_usage_workspace")]
    pub units_usage_workspace: Option<i64>,
    #[serde(default, rename = "usage_reset_date")]
    pub usage_reset_date: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LimitsAndUsageResponse {
    #[serde(default, rename = "limits_and_usage")]
    pub limits_and_usage: Option<LimitsAndUsage>,
}
