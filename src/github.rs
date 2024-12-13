use crate::types::{PullRequest, SearchResponse};
use reqwest::Client;
use std::error::Error;

const GITHUB_API_URL: &str = "https://api.github.com/search/issues";

pub async fn fetch_contributions(
    username: &str,
    page: u32,
) -> Result<SearchResponse, Box<dyn Error>> {
    let client = Client::new();
    let query = format!("author:{} is:pr is:merged", username);
    let url = format!("{}?q={}&per_page=10&page={}", GITHUB_API_URL, query, page);

    let response = client
        .get(&url)
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "Rust-Contributor-Fetcher")
        .send()
        .await?;

    let data = response.json::<SearchResponse>().await?;
    Ok(data)
}

pub async fn fetch_all_contributions(username: &str) -> Result<Vec<PullRequest>, Box<dyn Error>> {
    let mut page = 1;
    let mut all_prs = vec![];

    loop {
        let data = fetch_contributions(username, page).await?;
        if data.items.is_empty() {
            break;
        }
        all_prs.extend(data.items);
        page += 1;
    }

    Ok(all_prs)
}
