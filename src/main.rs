mod github;
mod output;
mod types;

use colored::*;
use github::fetch_all_contributions;
use output::print_contributions;
use std::{
    collections::HashSet,
    env,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

const USAGE_MESSAGE: &str = "Usage: cargo run <github_username> [repos.txt]";
const GITHUB_PREFIX: &str = "https://github.com/";
const GITHUB_API_PREFIX: &str = "https://api.github.com/repos/";

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{}: {}", "Error".bold().red(), e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let username = args.get(1).ok_or_else(|| {
        println!("{}", USAGE_MESSAGE.bold().yellow());
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Missing username")
    })?;

    println!("{}", format!("Fetching contributions for {}...", username).bold().blue());
    let mut all_prs = fetch_all_contributions(username).await?;

    if let Some(repos_file) = args.get(2).filter(|f| f.ends_with(".txt")) {
        match read_repos_file(repos_file) {
            Ok(repos_set) => {
                all_prs = filter_contributions(all_prs, &repos_set);
                println!(
                    "{} {}",
                    "Filtered PRs after repos.txt:".bold().cyan(),
                    all_prs.len().to_string().bold().green()
                );
            }
            Err(e) => println!("{}: {}", "Failed to read repos file".bold().yellow(), e),
        }
    }

    if all_prs.is_empty() {
        println!("{}", "No merged pull requests found.".bold().yellow());
    } else {
        println!(
            "\n{} {}",
            "Total Merged PRs:".bold().cyan(),
            all_prs.len().to_string().bold().green()
        );
        print_contributions(&all_prs);
    }

    Ok(())
}

fn read_repos_file(filepath: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let file = File::open(filepath)?;
    let reader = BufReader::new(file);
    let mut repos_set = HashSet::new();

    for line in reader.lines() {
        let repo_url = line?.trim().to_string();
        if repo_url.is_empty() {
            continue;
        }

        if let Some(stripped) = repo_url.strip_prefix(GITHUB_PREFIX) {
            repos_set.insert(stripped.to_string());
        }
    }

    Ok(repos_set)
}

fn filter_contributions(
    prs: Vec<types::PullRequest>,
    repos: &HashSet<String>,
) -> Vec<types::PullRequest> {
    prs.into_iter()
        .filter(|pr| {
            pr.repository_url
                .strip_prefix(GITHUB_API_PREFIX)
                .map_or(false, |repo| repos.contains(repo))
        })
        .collect()
}
