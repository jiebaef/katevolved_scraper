use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use reqwest::{Client, Response};
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Debug)]
struct Account {
    region: String,
    puuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Summoner {
    account_id: String,
    profile_icon_id: u32,
    revision_date: u64,
    name: String,
    id: String,
    puuid: String,
    summoner_level: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Match {
    metadata: Metadata,
    info: Info,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    data_version: String,
    match_id: String,
    participants: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Info {}
// #[derive(Debug, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct SOMESTRUCT {

// }
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let key = env::var("api_key").expect("\"api_key\" must be set");
    let accs = env::var("accs").expect("\"accs\" must be set, eg:\tregion:puuid|region:puuid");

    let accs: Vec<Account> = accs
        .split("|")
        .map(|pair| {
            let mut parts = pair.split(":");
            let region = parts.next().unwrap_or_default().to_string();
            let puuid = parts.next().unwrap_or_default().to_string();
            Account { region, puuid }
        })
        .collect();

    let client = Client::new();
    for acc in accs {
        let url = format!(
            "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}",
            acc.region, acc.puuid
        );

        let response = get(&client, &key, url).await?;
        if !response.status().is_success() {
            eprintln!("Request failed with status: {}", response.status());
            return Ok(());
        }
        let summoner: Summoner = response.json().await?;
        // println!("{}", summoner.puuid);
        // return Ok(());

        let url = format!(
            "https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids",
            summoner.puuid //"H7hZYwWJ-BhD8qEQ3anrGXW-HjU-EnWp_HBk4BvjyfL9fdIpnzvJce0zFmXa0d2iMawRHxjD9AsSJg"
        );
        let response = get(&client, &key, url).await?;
        if !response.status().is_success() {
            eprintln!("Request failed with status: {}", response.status());
            return Ok(());
        }
        let matches: Vec<String> = response.json().await?;
        // println!("{:?}", matches);

        for matchid in matches {
            let url = format!("{}", matchid);
            let response = get(&client, &key, url).await?;
            if !response.status().is_success() {
                eprintln!("Request failed with status: {}", response.status());
                return Ok(());
            }
            let lolmatch = response.json().await?;
        }
    }

    Ok(())
}

async fn get(client: &Client, api_key: &str, url: String) -> Result<Response, Box<dyn Error>> {
    return Ok(client
        .get(url)
        .header(USER_AGENT, "Rust App 1")
        .header("X-Riot-Token", api_key)
        .send()
        .await?);
}
