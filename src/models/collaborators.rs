use serde::{Deserialize, Serialize};

pub type Collaborators = Vec<Collaborator>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Collaborator {
    pub avatar_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub events_url: String,
    pub followers_url: String,
    pub following_url: String,
    pub gists_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gravatar_id: Option<String>,
    pub html_url: String,
    pub id: i64,
    pub login: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub node_id: String,
    pub organizations_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Permissions>,
    pub received_events_url: String,
    pub repos_url: String,
    pub role_name: String,
    pub site_admin: bool,
    pub starred_url: String,
    pub subscriptions_url: String,
    #[serde(rename = "type")]
    pub collaborator_type: String,
    pub url: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub admin: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maintain: Option<bool>,
    pub pull: bool,
    pub push: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triage: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_data() {
        const DATA: &str = r#"
            [
                {
                    "login": "octocat",
                    "id": 1,
                    "node_id": "MDQ6VXNlcjE=",
                    "avatar_url": "https://github.com/images/error/octocat_happy.gif",
                    "gravatar_id": "",
                    "url": "https://api.github.com/users/octocat",
                    "html_url": "https://github.com/octocat",
                    "followers_url": "https://api.github.com/users/octocat/followers",
                    "following_url": "https://api.github.com/users/octocat/following{/other_user}",
                    "gists_url": "https://api.github.com/users/octocat/gists{/gist_id}",
                    "starred_url": "https://api.github.com/users/octocat/starred{/owner}{/repo}",
                    "subscriptions_url": "https://api.github.com/users/octocat/subscriptions",
                    "organizations_url": "https://api.github.com/users/octocat/orgs",
                    "repos_url": "https://api.github.com/users/octocat/repos",
                    "events_url": "https://api.github.com/users/octocat/events{/privacy}",
                    "received_events_url": "https://api.github.com/users/octocat/received_events",
                    "type": "User",
                    "site_admin": false,
                    "permissions": {
                        "pull": true,
                        "triage": true,
                        "push": true,
                        "maintain": false,
                        "admin": false
                    },
                    "role_name": "write"
                }
            ]
        "#;

        let collaborators: Collaborators = serde_json::from_str(DATA).unwrap();

        assert_eq!(collaborators.len(), 1);
    }
}
