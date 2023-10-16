use dotenv::dotenv;
use std::env;
use std::error::Error;

use cfbd::{get_endpoint_data, Endpoint};

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    //set up env vars
    dotenv().ok();
    let api_token = std::env::var("CFB_API_TOKEN").expect("CFB_API_TOKEN must be set!");

    let _team_data = get_endpoint_data(Endpoint::Teams(Some("b1g")), &api_token)?;

    Ok(())
}
