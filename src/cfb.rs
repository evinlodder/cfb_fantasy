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

#[derive(Serialize, Deserialize, Debug)]
pub struct TeamLocation {
    pub venue_id: Option<u32>,
    pub name: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip: Option<String>,
    pub country_code: Option<String>,
    pub timezone: Option<String>,
    pub latitude: Option<f32>,
    pub longitude: Option<f32>,
    pub elevation: Option<String>,
    pub capacity: Option<u32>,
    pub year_constructed: Option<u16>,
    pub grass: Option<bool>,
    pub dome: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Team {
    pub id: u32,
    pub school: String,
    pub mascot: Option<String>,
    pub abbreviation: Option<String>,
    pub alt_name1: Option<String>,
    pub alt_name2: Option<String>,
    pub alt_name3: Option<String>,
    pub conference: Option<String>,
    pub classification: Option<String>,
    pub color: Option<String>,
    pub alt_color: Option<String>,
    pub logos: Option<Vec<String>>,
    pub twitter: Option<String>,
    pub location: TeamLocation,
}

#[allow(non_snake_case)] //only because that's how the public API has it
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerStats {
    pub playerId: u64,
    pub player: String,
    pub team: String,
    pub conference: String,
    pub category: String,
    pub statType: String,
    pub stat: u32,
}

pub enum StatCategory {
    Passing,
    Rushing,
    Receiving,
    Defensive,
}

pub enum SeasonType {
    Regular,
    Postseason,
    Both,
}

pub struct SeasonStatsParameters {
    pub team: Option<String>,
    pub conference: Option<String>,
    pub start_week: Option<u8>,
    pub end_week: Option<u8>,
    pub season_type: Option<SeasonType>,
    pub category: Option<StatCategory>,
}

pub enum Endpoint<'a> {
    Conferences,
    Teams(String),
    SeasonStats(u16, Option<&'a SeasonStatsParameters>),
}

impl<'a> fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match &*self {
            Endpoint::Conferences => String::from("conferences"),
            Endpoint::Teams(conf) => format!("teams?conference={}", conf),
            Endpoint::SeasonStats(year, opts) => {
                let mut s = format!("stats/player/season?year={year}");
                if opts.is_none() {
                    s
                } else {
                    let opts = opts.unwrap();

                    if let Some(team) = &opts.team {
                        s += &format!("&team={}", team.replace(" ", "%20"));
                    }
                    if let Some(conference) = &opts.conference {
                        s += &format!("&conference={}", conference.replace(" ", "%20"));
                    }
                    if let Some(start_week) = &opts.start_week {
                        s += &format!("&startWeek={start_week}");
                    }
                    if let Some(end_week) = &opts.end_week {
                        s += &format!("&endWeek={end_week}");
                    }
                    if let Some(season_type) = &opts.season_type {
                        let season_type = match season_type {
                            SeasonType::Regular => "regular",
                            SeasonType::Postseason => "postseason",
                            SeasonType::Both => "both",
                        };
                        s += &format!("&seasonType={season_type}");
                    }
                    if let Some(category) = &opts.category {
                        let category = match category {
                            StatCategory::Passing => "passing",
                            StatCategory::Receiving => "receiving",
                            StatCategory::Rushing => "rushing",
                            StatCategory::Defensive => "defensive",
                        };
                        s += &format!("&category={category}");
                    }

                    s
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_season_type_stattype_season_stats() {
        let parameters = SeasonStatsParameters {
            team: None,
            conference: None,
            start_week: None,
            end_week: None,
            season_type: Some(SeasonType::Regular),
            category: Some(StatCategory::Passing),
        };
        let endpoint = Endpoint::SeasonStats(2003, Some(&parameters));
        assert_eq!(
            format!("{endpoint}"),
            String::from("stats/player/season?year=2003&seasonType=regular&category=passing")
        );
    }
    #[test]
    fn test_empty_season_stats() {
        let parameters = SeasonStatsParameters {
            team: None,
            conference: None,
            start_week: None,
            end_week: None,
            season_type: None,
            category: None,
        };
        let endpoint = Endpoint::SeasonStats(2003, Some(&parameters));
        assert_eq!(
            format!("{endpoint}"),
            String::from("stats/player/season?year=2003")
        );
    }
}
