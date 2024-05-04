use serde::{Deserialize, Serialize};

pub type Collaborators = Vec<Collaborator>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Collaborator {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    #[serde(rename = "type")]
    collaborator_type: String,
    site_admin: bool,
    permissions: Permissions,
    role_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Permissions {
    pull: bool,
    triage: bool,
    push: bool,
    maintain: bool,
    admin: bool,
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
