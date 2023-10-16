use serde::{Deserialize, Serialize};
use std::fmt;

// Datatypes that are used in deserialization of API data

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Classification {
    Fbs,
    Fcs,
    II,
    III,
}

///`Conference` ontains summary data for conferences (e.g. SEC, Big Ten, ACC...)
///
///`classification` can be one of four values:
/// "fbs" - best D1 schools (OSU, Alabama, etc.)
/// "fcs" - lower-end D1 schools (Harvard, Dartmouth, etc.)
/// "ii"  - D2 schools
/// "iii" - D3 schools
#[derive(Serialize, Deserialize, PartialEq)]
pub struct Conference {
    pub id: u32,
    pub name: String,
    pub short_name: String,
    pub abbreviation: Option<String>,
    pub classification: Classification,
}

///`TeamLocation` contains summary data for team locations.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

///`Team` ontains summary data for teams.
///
///For smaller schools, the only fields set are `id` and `school`.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Team {
    pub id: u32,
    pub school: String,
    pub mascot: Option<String>,
    pub abbreviation: Option<String>,
    pub alt_name1: Option<String>,
    pub alt_name2: Option<String>,
    pub alt_name3: Option<String>,
    pub conference: Option<String>,
    pub classification: Option<Classification>,
    pub color: Option<String>,
    pub alt_color: Option<String>,
    pub logos: Option<Vec<String>>,
    pub twitter: Option<String>,
    pub location: TeamLocation,
}

///`PlayerStats` contains summary data for a single statistic recorded by a player in a game
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PlayerStats {
    #[serde(rename = "playerId")]
    pub player_id: String,
    pub player: String,
    pub team: String,
    pub conference: String,
    pub category: StatCategory,
    #[serde(rename = "statType")]
    pub stat_type: String,
    pub stat: String,
}

///`StatCategory` is an enum for the stat category for use when searching for stats
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StatCategory {
    Passing,
    Rushing,
    Receiving,
    Defensive,
    Interceptions,
    Kicking,
    #[serde(alias = "kickReturns")]
    KickReturns,
    #[serde(alias = "puntReturns")]
    PuntReturns,
    Punting,
}

// Datatypes that are not used in deserialization of API data

pub enum SeasonType {
    Regular,
    Postseason,
    Both,
}

pub struct SeasonStatsParameters<'a> {
    pub team: Option<&'a str>,
    pub conference: Option<&'a str>,
    pub start_week: Option<u8>,
    pub end_week: Option<u8>,
    pub season_type: Option<SeasonType>,
    pub category: Option<StatCategory>,
}

pub enum Endpoint<'a> {
    Conferences,
    Teams(Option<&'a str>),
    SeasonStats(u16, Option<&'a SeasonStatsParameters<'a>>),
}

impl<'a> fmt::Display for Endpoint<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = match self {
            Endpoint::Conferences => String::from("conferences"),
            Endpoint::Teams(conf) => {
                if conf.is_none() {
                    "teams".to_owned()
                } else {
                    format!("teams?conference={}", conf.unwrap())
                }
            }
            Endpoint::SeasonStats(year, opts) => {
                let mut s = format!("stats/player/season?year={year}");
                if opts.is_none() {
                    s
                } else {
                    let opts = opts.unwrap();

                    if let Some(team) = &opts.team {
                        s += &format!("&team={}", team.replace(' ', "%20"));
                    }
                    if let Some(conference) = &opts.conference {
                        s += &format!("&conference={}", conference.replace(' ', "%20"));
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
                            StatCategory::Interceptions => "interceptions",
                            StatCategory::Kicking => "kicking",
                            StatCategory::KickReturns => "kickReturns",
                            StatCategory::PuntReturns => "puntReturns",
                            StatCategory::Punting => "punting",
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
