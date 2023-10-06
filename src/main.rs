use curl::easy::{Easy, List};
use curl::Error;
use dotenv::dotenv;
use std::env;

mod cfb;
use cfb::{get_data, Conference, Endpoint, Team};

fn main() -> Result<(), Error> {
    env::set_var("RUST_BACKTRACE", "1");
    //set up env vars
    dotenv().ok();
    let api_token = std::env::var("CFB_API_TOKEN").expect("CFB_API_TOKEN must be set!");

    let conference_data = get_data(Endpoint::Conferences, &api_token)?;

    let conferences: Vec<Conference> = serde_json::from_slice(&conference_data).unwrap();

    let team_data = get_data(Endpoint::Teams("ACC".to_owned()), &api_token)?;
    let acc_teams: Vec<Team> = serde_json::from_slice(&team_data).unwrap();
    Ok(())
}
