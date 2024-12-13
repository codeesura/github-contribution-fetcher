use crate::types::PullRequest;

pub fn print_contributions(prs: &[PullRequest]) {
    let mut all_prs = prs.to_vec();

    all_prs.sort_by(|a, b| {
        let a_date = a.pull_request.merged_at.as_ref().unwrap_or(&a.created_at);
        let b_date = b.pull_request.merged_at.as_ref().unwrap_or(&b.created_at);
        b_date.cmp(a_date)
    });

    let repo_width = 30;
    let link_width = 80;
    let date_width = 25;

    let separator = format!(
        "+{}+{}+{}+",
        "-".repeat(repo_width),
        "-".repeat(link_width),
        "-".repeat(date_width)
    );

    println!();
    println!("Contributions Table:\n");

    println!("{}", separator);
    println!(
        "|{:^repo_width$}|{:^link_width$}|{:^date_width$}|",
        "Repo",
        "PR Link",
        "Date",
        repo_width = repo_width,
        link_width = link_width,
        date_width = date_width
    );
    println!("{}", separator);

    for pr in all_prs {
        // Convert repository_url (e.g. "https://api.github.com/repos/user/repo") to "user/repo"
        let repo_name = pr
            .repository_url
            .strip_prefix("https://api.github.com/repos/")
            .unwrap_or(&pr.repository_url);

        let date = pr.pull_request.merged_at.as_ref().unwrap_or(&pr.created_at);

        let truncated_repo = truncate(repo_name, repo_width);
        let truncated_link = truncate(&pr.html_url, link_width);
        let truncated_date = truncate(date, date_width);

        println!(
            "|{:repo_width$}|{:link_width$}|{:date_width$}|",
            truncated_repo,
            truncated_link,
            truncated_date,
            repo_width = repo_width,
            link_width = link_width,
            date_width = date_width
        );
    }

    println!("{}", separator);
    println!();
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut truncated = s[..max_len - 3].to_string();
        truncated.push_str("...");
        truncated
    }
}
