mod riotdtos;
mod structs;

use dotenv::dotenv;
use reqwest::header::USER_AGENT;
use reqwest::{Client, Response};
use riotdtos::{Match, Summoner};
use std::error::Error;
use std::{env, vec};
use structs::Account;

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

    let games = get_matches(accs, &key).await?;

    println!("{:#?}", games);

    Ok(())
}

async fn get_matches(accs: Vec<Account>, key: &str) -> Result<Vec<Match>, reqwest::Error> {
    let client = Client::new();
    let mut games: Vec<Match> = vec![];
    for acc in accs {
        let url = format!(
            "https://{}.api.riotgames.com/lol/summoner/v4/summoners/by-puuid/{}",
            acc.region, acc.puuid
        );

        let response = get(&client, &key, url).await?;
        response.error_for_status_ref()?;
        let summoner: Summoner = response.json().await?;

        let url = format!(
            "https://americas.api.riotgames.com/lol/match/v5/matches/by-puuid/{}/ids",
            summoner.puuid
        );
        let response = get(&client, &key, url).await?;
        response.error_for_status_ref()?;
        let matches: Vec<String> = response.json().await?;

        for matchid in matches {
            let url = format!(
                "https://americas.api.riotgames.com/lol/match/v5/matches/{}",
                matchid
            );
            let response = get(&client, &key, url).await?;
            response.error_for_status_ref()?;
            let lolmatch: Match = response.json().await?;
            games.push(lolmatch);
        }
    }

    Ok(games)
}

async fn get(client: &Client, api_key: &str, url: String) -> Result<Response, reqwest::Error> {
    return Ok(client
        .get(url)
        .header(USER_AGENT, "Rust App 1")
        .header("X-Riot-Token", api_key)
        .send()
        .await?);
}

// fn fixSerdeNaming() {
//     let path = "H:/Rust/katevolved_scraper/src/match.json";
//     let mut file = std::fs::File::open(path).expect("Failed to open file");
//     let mut contents = String::new();
//     file.read_to_string(&mut contents)
//         .expect("error when reading file");
//     let parsed: Match = serde_json::from_str(&contents).expect("Failed parsing");
//     println!("{:?}", parsed);
// }
