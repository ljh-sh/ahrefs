//! Response models for the `social-media` API group.

#![allow(missing_docs)]

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActivitiesItem {
    #[serde(default, rename = "action_type")]
    pub action_type: Option<String>,
    #[serde(default, rename = "activity_id")]
    pub activity_id: Option<i64>,
    #[serde(default, rename = "channel_id")]
    pub channel_id: Option<String>,
    #[serde(default, rename = "channel_kind")]
    pub channel_kind: Option<String>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "user_name")]
    pub user_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActivityHistoryResponse {
    #[serde(default, rename = "activities")]
    pub activities: Option<Vec<ActivitiesItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthorsItem {
    #[serde(default, rename = "author_id")]
    pub author_id: Option<String>,
    #[serde(default, rename = "author_name")]
    pub author_name: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AuthorsResponse {
    #[serde(default, rename = "authors")]
    pub authors: Option<Vec<AuthorsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem {
    #[serde(default, rename = "follower_count")]
    pub follower_count: Option<i64>,
    #[serde(default, rename = "retrieved_at")]
    pub retrieved_at: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelMetricsResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelsItem {
    #[serde(default, rename = "channel_display_name")]
    pub channel_display_name: Option<String>,
    #[serde(default, rename = "channel_id")]
    pub channel_id: Option<String>,
    #[serde(default, rename = "channel_kind")]
    pub channel_kind: Option<String>,
    #[serde(default, rename = "channel_profile_image_url")]
    pub channel_profile_image_url: Option<String>,
    #[serde(default, rename = "channel_status")]
    pub channel_status: Option<String>,
    #[serde(default, rename = "channel_username")]
    pub channel_username: Option<String>,
    #[serde(default, rename = "external_channel_id")]
    pub external_channel_id: Option<String>,
    #[serde(default, rename = "token_validity")]
    pub token_validity: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelsResponse {
    #[serde(default, rename = "channels")]
    pub channels: Option<Vec<ChannelsItem>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelsItem2 {
    #[serde(default, rename = "channel_id")]
    pub channel_id: Option<String>,
    #[serde(default, rename = "channel_kind")]
    pub channel_kind: Option<String>,
    #[serde(default, rename = "channel_username")]
    pub channel_username: Option<String>,
    #[serde(default, rename = "channel_display_name")]
    pub channel_display_name: Option<String>,
    #[serde(default, rename = "channel_profile_image_url")]
    pub channel_profile_image_url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Post {
    #[serde(default, rename = "author_id")]
    pub author_id: Option<String>,
    #[serde(default, rename = "author_name")]
    pub author_name: Option<String>,
    #[serde(default, rename = "auto_comment")]
    pub auto_comment: Option<String>,
    #[serde(default, rename = "channels")]
    pub channels: Option<Vec<ChannelsItem2>>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "external_post_id")]
    pub external_post_id: Option<String>,
    #[serde(default, rename = "permalink")]
    pub permalink: Option<String>,
    #[serde(default, rename = "post_id")]
    pub post_id: Option<i64>,
    #[serde(default, rename = "status_timestamp")]
    pub status_timestamp: Option<String>,
    #[serde(default, rename = "text_content")]
    pub text_content: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostResponse {
    #[serde(default, rename = "post")]
    pub post: Option<Post>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FacebookMetrics {
    #[serde(default, rename = "comments")]
    pub comments: Option<i64>,
    #[serde(default, rename = "reactions")]
    pub reactions: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstagramMetrics {
    #[serde(default, rename = "comments")]
    pub comments: Option<i64>,
    #[serde(default, rename = "shares")]
    pub shares: Option<i64>,
    #[serde(default, rename = "saved")]
    pub saved: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LinkedinMetrics {
    #[serde(default, rename = "comments")]
    pub comments: Option<i64>,
    #[serde(default, rename = "reshares")]
    pub reshares: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ThreadsMetrics {
    #[serde(default, rename = "replies")]
    pub replies: Option<i64>,
    #[serde(default, rename = "reposts")]
    pub reposts: Option<i64>,
    #[serde(default, rename = "quotes")]
    pub quotes: Option<i64>,
    #[serde(default, rename = "shares")]
    pub shares: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TiktokMetrics {
    #[serde(default, rename = "comments")]
    pub comments: Option<i64>,
    #[serde(default, rename = "shares")]
    pub shares: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TwitterMetrics {
    #[serde(default, rename = "replies")]
    pub replies: Option<i64>,
    #[serde(default, rename = "retweets")]
    pub retweets: Option<i64>,
    #[serde(default, rename = "quote_tweets")]
    pub quote_tweets: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct YoutubeMetrics {
    #[serde(default, rename = "comments")]
    pub comments: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MetricsItem2 {
    #[serde(default, rename = "facebook_metrics")]
    pub facebook_metrics: Option<FacebookMetrics>,
    #[serde(default, rename = "instagram_metrics")]
    pub instagram_metrics: Option<InstagramMetrics>,
    #[serde(default, rename = "likes")]
    pub likes: Option<i64>,
    #[serde(default, rename = "linkedin_metrics")]
    pub linkedin_metrics: Option<LinkedinMetrics>,
    #[serde(default, rename = "retrieved_at")]
    pub retrieved_at: Option<String>,
    #[serde(default, rename = "threads_metrics")]
    pub threads_metrics: Option<ThreadsMetrics>,
    #[serde(default, rename = "tiktok_metrics")]
    pub tiktok_metrics: Option<TiktokMetrics>,
    #[serde(default, rename = "twitter_metrics")]
    pub twitter_metrics: Option<TwitterMetrics>,
    #[serde(default, rename = "unique_views")]
    pub unique_views: Option<i64>,
    #[serde(default, rename = "views")]
    pub views: Option<i64>,
    #[serde(default, rename = "youtube_metrics")]
    pub youtube_metrics: Option<YoutubeMetrics>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostMetricsResponse {
    #[serde(default, rename = "metrics")]
    pub metrics: Option<Vec<MetricsItem2>>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChannelsItem3 {
    #[serde(default, rename = "channel_id")]
    pub channel_id: Option<String>,
    #[serde(default, rename = "channel_kind")]
    pub channel_kind: Option<String>,
    #[serde(default, rename = "channel_username")]
    pub channel_username: Option<String>,
    #[serde(default, rename = "channel_display_name")]
    pub channel_display_name: Option<String>,
    #[serde(default, rename = "channel_profile_image_url")]
    pub channel_profile_image_url: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostsItem {
    #[serde(default, rename = "author_id")]
    pub author_id: Option<String>,
    #[serde(default, rename = "author_name")]
    pub author_name: Option<String>,
    #[serde(default, rename = "auto_comment")]
    pub auto_comment: Option<String>,
    #[serde(default, rename = "channels")]
    pub channels: Option<Vec<ChannelsItem3>>,
    #[serde(default, rename = "created_at")]
    pub created_at: Option<String>,
    #[serde(default, rename = "external_post_id")]
    pub external_post_id: Option<String>,
    #[serde(default, rename = "permalink")]
    pub permalink: Option<String>,
    #[serde(default, rename = "post_id")]
    pub post_id: Option<i64>,
    #[serde(default, rename = "status_timestamp")]
    pub status_timestamp: Option<String>,
    #[serde(default, rename = "text_content")]
    pub text_content: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostsResponse {
    #[serde(default, rename = "posts")]
    pub posts: Option<Vec<PostsItem>>,
}
