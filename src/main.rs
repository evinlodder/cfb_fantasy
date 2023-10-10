use dotenv::dotenv;
use std::env;
use std::error::Error;

mod cfb;
use cfb::{get_data, Endpoint};

fn main() -> Result<(), Box<dyn Error>> {
    env::set_var("RUST_BACKTRACE", "1");
    //set up env vars
    dotenv().ok();
    let api_token = std::env::var("CFB_API_TOKEN").expect("CFB_API_TOKEN must be set!");

    let team_data = get_data(Endpoint::Teams("b1g".to_owned()), &api_token)?;

    Ok(())
}
