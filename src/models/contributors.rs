use serde::{Deserialize, Serialize};

pub type Contributors = Vec<Contributor>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contributor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    pub contributions: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub followers_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub following_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gists_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_events_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repos_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub site_admin: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions_url: Option<String>,
    #[serde(rename = "type")]
    pub contributor_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
