use clap::{Arg, Command};
use reqwest::{Client, Error};
use serde::Deserialize;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = Command::new("GitHub-Activity")
        .version("1.0")
        .author("loser_smurf")
        .about("A simple CLI tool with one argument to get request to github")
        .arg(
            Arg::new("username")
                .help("Enter a github username")
                .required(true),
        )
        .get_matches();

    let name = matches.get_one::<String>("username").expect("Username is required");
    println!("Hi, {name}");
    let events_summary = make_request(&name).await?;
    println!("{:#?}", events_summary); 
    Ok(())
}

async fn make_request(username: &str) -> Result<HashMap<String, i32>, Error> {
    let url = format!("https://api.github.com/users/{}/events", username);
    
    let client = Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "Rust-Client")
        .send()
        .await?;

    let status = response.status();
    let body = response.text().await?;
    
    let events_summary = if status.is_success() {
        match serde_json::from_str::<Vec<Event>>(&body) {
            Ok(events) => {
                let mut result = HashMap::new();
                for event in events {
                    match event.event_type.as_str() {
                        "PushEvent" => {
                            *result.entry("PushEvent".to_string()).or_insert(0) += 1;
                        }
                        "IssuesEvent" => {
                            *result.entry("IssuesEvent".to_string()).or_insert(0) += 1;
                        }
                        "WatchEvent" => {
                            *result.entry("WatchEvent".to_string()).or_insert(0) += 1;
                        }
                        _ => {}
                    }
                }
                result
            }
            Err(err) => {
                eprintln!("Failed to parse JSON response: {}", err);
                eprintln!("Raw response body: {}", body);
                HashMap::new()
            }
        }
    } else {
        eprintln!("Request failed with status: {}", status);
        eprintln!("Raw response body: {}", body);
        HashMap::new()
    };

    Ok(events_summary)
}

#[derive(Debug, Deserialize)]
struct Event {
    id: String,
    #[serde(rename = "type")]
    event_type: String,
    actor: Actor,
    repo: Repo,
    payload: Option<Payload>,
    public: bool,
    created_at: String,
}

#[derive(Debug, Deserialize)]
struct Actor {
    id: u64,
    login: String,
    display_login: Option<String>,
    gravatar_id: Option<String>,
    url: String,
    avatar_url: String,
}

#[derive(Debug, Deserialize)]
struct Repo {
    id: u64,
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct Payload {
    #[serde(rename = "ref")]
    ref_field: Option<String>,
    head: Option<String>,
    before: Option<String>,
    size: Option<u32>,
    distinct_size: Option<u32>,
    commits: Option<Vec<Commit>>,
}

#[derive(Debug, Deserialize)]
struct Commit {
    sha: Option<String>,
    author: Option<Author>,
    message: Option<String>,
    distinct: Option<bool>,
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Author {
    email: Option<String>,
    name: Option<String>,
}
