use std::error::Error;

use reqwest::blocking::get;

fn main() -> Result<(), Box<dyn Error>> {
    let response = get("https://google.com")?;

    if response.status().is_success() {
        let body = response.text()?;
        println!("Response body: {body}")
    } else {
        eprintln!("Request failed with status: {}", response.status())
    }

    Ok(())
}
