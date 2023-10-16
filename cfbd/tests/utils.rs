use cfbd::*;
use dotenv::dotenv;
use serde_json::from_slice;

fn token() -> String {
    dotenv().ok();
    std::env::var("CFB_API_TOKEN").expect("CFB_API_TOKEN must be set!")
}

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

#[test]
fn test_conference_endpoint() {
    let token = token();
    let data = get_endpoint_data(Endpoint::Conferences, &token).unwrap();
    let conferences: Vec<Conference> = serde_json::from_slice(&data).unwrap();

    assert!(conferences.contains(&Conference {
        id: 1,
        name: "ACC".to_owned(),
        short_name: "Atlantic Coast Conference".to_owned(),
        abbreviation: Some("ACC".to_owned()),
        classification: Classification::Fbs,
    }))
}

#[test]
fn test_teams_endpoint() {
    let token = token();
    let data = get_endpoint_data(Endpoint::Teams(Some("SEC")), &token).unwrap();
    let conferences: Vec<Team> = serde_json::from_slice(&data).unwrap();

    assert_eq!(
        conferences.first().unwrap(),
        &Team {
            id: 333,
            school: "Alabama".to_owned(),
            mascot: Some("Crimson Tide".to_owned()),
            abbreviation: Some("ALA".to_owned()),
            alt_name1: None,
            alt_name2: Some("ALA".to_owned()),
            alt_name3: Some("Alabama".to_owned()),
            conference: Some("SEC".to_owned()),
            classification: Some(Classification::Fbs),
            color: Some("#690014".to_owned()),
            alt_color: Some("#f1f2f3".to_owned()),
            logos: Some(vec![
                "http://a.espncdn.com/i/teamlogos/ncaa/500/333.png".to_owned(),
                "http://a.espncdn.com/i/teamlogos/ncaa/500-dark/333.png".to_owned()
            ]),
            twitter: Some("@AlabamaFTBL".to_owned()),
            location: TeamLocation {
                venue_id: Some(3657),
                name: Some("Bryant Denny Stadium".to_owned()),
                city: Some("Tuscaloosa".to_owned()),
                state: Some("AL".to_owned()),
                zip: Some("35487".to_owned()),
                country_code: Some("US".to_owned()),
                timezone: Some("America/Chicago".to_owned()),
                latitude: Some(33.2082752 as f32),
                longitude: Some(-87.5503836 as f32),
                elevation: Some("70.05136108".to_owned()),
                capacity: Some(101821),
                year_constructed: Some(1929),
                grass: Some(true),
                dome: Some(false),
            },
        }
    )
}

#[test]
fn test_season_stats_endpoint() {
    let token = token();
    let data = get_endpoint_data(
        Endpoint::SeasonStats(
            2022,
            Some(&SeasonStatsParameters {
                team: Some("Alabama"),
                conference: None,
                start_week: Some(1),
                end_week: Some(1),
                season_type: None,
                category: None,
            }),
        ),
        &token,
    )
    .unwrap();
    let conferences: Vec<PlayerStats> = serde_json::from_slice(&data).unwrap();

    assert_eq!(
        conferences.first().unwrap(),
        &PlayerStats {
            player_id: "4428989".to_owned(),
            player: "Chris Braswell".to_owned(),
            team: "Alabama".to_owned(),
            conference: "SEC".to_owned(),
            category: StatCategory::Defensive,
            stat_type: "QB HUR".to_owned(),
            stat: "0".to_owned(),
        }
    )
}
