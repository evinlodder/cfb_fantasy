use curl::easy::{Easy, List};
use dotenv::dotenv;

mod cfb;
use cfb::Conference;

fn main() {
    //set up env vars
    dotenv().ok();
    let api_token = std::env::var("CFB_API_TOKEN").expect("CFB_API_TOKEN must be set!");

    //testing out lib
    let mut curl_data = Vec::new();

    //curl client
    let mut easy = Easy::new();
    easy.url("https://api.collegefootballdata.com/conferences")
        .unwrap();
    //set headers

    let mut list = List::new();

    let auth = format!("Authorization: Bearer {api_token}");

    list.append(&auth).unwrap();
    list.append("accept: application/json").unwrap();
    easy.http_headers(list).unwrap();

    let mut transfer = easy.transfer();
    //set request's callback
    transfer
        .write_function(|data| {
            curl_data.extend_from_slice(data);
            Ok(data.len())
        })
        .unwrap();

    transfer.perform().unwrap();
    drop(transfer);

    let conferences: Vec<Conference> = serde_json::from_slice(&curl_data).unwrap();

    for conference in &conferences {
        println!("{}", conference.name);
    }
}
