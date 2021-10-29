use dotenv;
use octocrab::{self, models::Repository, Octocrab, Page};
use serde_json::Value;

fn build_octocrab() -> Octocrab {
    let access_token = std::env::var("GITHUB_PERSONAL_ACCESS_TOKEN")
        .expect("GITHUB_PERSONAL_ACCESS_TOKEN env var must be set");
    Octocrab::builder()
        .personal_token(access_token)
        .build()
        .expect("Failed to build Octocrab")
}

async fn find_org_repo(octo: &Octocrab, organization: &str, repo_name: &str) -> Option<Repository> {
    match octo.orgs(organization).list_repos().send().await {
        Ok(p) => find_repo_in_pages(octo, p, repo_name).await,
        Err(e) => {
            eprintln!("Error: {}", e);
            return None;
        }
    }
}

async fn check_codeowners(octo: &Octocrab, org: &str) -> () {
    let mut repo_page = match octo.orgs(org).list_repos().send().await {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            return ();
        }
    };

    loop {
        for repo in repo_page.items.iter() {
            println!("{}", repo.name);
            match get_content(octo, repo, "CODEOWNERS").await {
                Ok(Some(s)) => {
                    let mut res: Vec<String> = vec![];
                    for line in s.split("\\n") {
                        let decoded = base64::decode(line).unwrap();
                        let owner = std::str::from_utf8(&decoded).unwrap();
                        res.push(owner.to_string());
                    }
                    println!("codeowners:\n{}", res.join(""));
                }
                Ok(None) => {
                    println!("No codeowner");
                }
                Err(s) => {
                    eprintln!("Error {}", s);
                    continue;
                }
            };
            println!("");
        }

        if let Some(p) = octo.get_page(&repo_page.next).await.unwrap() {
            repo_page = p;
        } else {
            return ();
        }
    }
}

async fn find_repo_in_pages(
    octo: &Octocrab,
    page: Page<Repository>,
    repo_name: &str,
) -> Option<Repository> {
    let mut repo_page = page;
    loop {
        let mut res = None;
        for repo in repo_page.items {
            if repo.name == repo_name || repo.full_name == repo_name {
                res = Some(repo);
                break;
            }
        }

        if res.is_some() {
            return res;
        } else {
            if let Some(p) = octo.get_page(&repo_page.next).await.unwrap() {
                repo_page = p;
            } else {
                return None;
            }
        }
    }
}

use std::collections::hash_map::HashMap;

async fn get_content(
    octo: &Octocrab,
    repo: &Repository,
    path: &str,
) -> Result<Option<String>, String> {
    let h: HashMap<String, String> = HashMap::new();
    match repo.contents_url.join(path) {
        Ok(content_url) => octo
            ._get(content_url, Some(&h))
            .await
            .unwrap()
            .json()
            .await
            .map(|resp: Value| match resp["content"].to_string().trim() {
                "null" => None,
                s => Some(
                    s.trim_start_matches("\"")
                        .trim_end_matches("\"")
                        .to_string(),
                ),
            })
            .map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let octo = build_octocrab();

    check_codeowners(&octo, "<some-org-here>").await;
    println!("Searching for repo.");
    let r = find_org_repo(&octo, "<some-org-here>", "<some-repo-name-here>").await;
    println!("Found: {:?}", r);

    println!("Searching for non-existent repo.");
    let r2 = find_org_repo(&octo, "<some-org-here>", "this_does_not_exist").await;
    println!("Found: {:?}", r2);
}
