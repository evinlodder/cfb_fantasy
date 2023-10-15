use crate::response_types::*;
use curl::easy::{Easy, List};
use curl::Error;

pub fn get_data(endpoint: Endpoint, api_key: &str) -> Result<Vec<u8>, Error> {
    //create return vector and curl client
    let mut curl_data = Vec::new();
    let mut easy = Easy::new();
    //set client url and headers
    let url = format!("https://api.collegefootballdata.com/{}", endpoint);
    easy.url(&url)?;
    let mut list = List::new();
    list.append("accept: application/json")?;
    list.append(&format!("Authorization: Bearer {api_key}"))?;
    easy.http_headers(list)?;
    //transfer the client so we can use the callback to edit curl_data
    let mut transfer = easy.transfer();
    transfer.write_function(|data| {
        curl_data.extend_from_slice(data);
        Ok(data.len())
    })?;
    transfer.perform()?;
    drop(transfer);
    Ok(curl_data)
}
