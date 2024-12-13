mod github;
mod output;
mod types;

use colored::*;
use github::fetch_all_contributions;
use output::print_contributions;
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    let username = match args.get(1) {
        Some(name) => name,
        None => {
            println!("{}", "Usage: cargo run <github_username>".bold().yellow());
            return;
        }
    };

    println!("{}", format!("Fetching contributions for {}...", username).bold().blue());

    match fetch_all_contributions(username).await {
        Ok(all_prs) => {
            println!(
                "\n{} {}",
                "Total Merged PRs:".bold().cyan(),
                all_prs.len().to_string().bold().green()
            );

            if !all_prs.is_empty() {
                print_contributions(&all_prs);
            } else {
                println!("{}", "No merged pull requests found.".bold().yellow());
            }
        }
        Err(e) => {
            eprintln!("{}: {}", "Error".bold().red(), e);
        }
    }
}