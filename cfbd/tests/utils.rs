use cfbd::*;

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
