use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct PullRequest {
    pub html_url: String,
    pub repository_url: String,
    pub pull_request: PullRequestDetails,
    pub created_at: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct PullRequestDetails {
    pub merged_at: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct SearchResponse {
    pub items: Vec<PullRequest>,
}
