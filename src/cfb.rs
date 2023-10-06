use curl::easy::{Easy, List};
use curl::Error;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Conference {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub abbreviation: Option<String>,
    pub classification: String,
}

#[derive(Serialize, Deserialize)]
pub struct TeamLocation {
    pub venue_id: u32,
    pub name: String,
    pub city: String,
    pub state: String,
    pub zip: String,
    pub country_code: String,
    pub timezone: String,
    pub latitude: f32,
    pub longitude: f32,
    pub elevation: String,
    pub capacity: u32,
    pub year_constructed: u16,
    pub grass: bool,
    pub dome: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Team {
    pub id: u32,
    pub school: String,
    pub mascot: String,
    pub abbreviation: String,
    pub alt_name1: Option<String>,
    pub alt_name2: Option<String>,
    pub alt_name3: Option<String>,
    pub conference: String,
    pub classification: String,
    pub color: String,
    pub alt_color: String,
    pub logos: Vec<String>,
    pub twitter: String,
    pub location: TeamLocation,
}

#[derive(Debug)]
pub enum Endpoint {
    Conferences,
    Teams(String),
}

impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match &*self {
            Endpoint::Conferences => format!("{:?}", self).to_lowercase(),
            Endpoint::Teams(conf) => format!("{:?}?conference={}", self, conf)
                .split("(")
                .next()
                .unwrap()
                .to_lowercase(),
        };
        write!(f, "{}", &s)
    }
}

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
