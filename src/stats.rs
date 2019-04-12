use crate::league::{roster, scoring};
use clap::ArgMatches;
use serde::{Deserialize, Serialize};
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;

mod schedule;
mod sportradar;

pub struct Config {
    date: String,
    league: String,
    api_key: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Player {
    name: String,
    position: String,
    primary_position: String,
    batter_stats: Option<BatterStats>,
    pitcher_stats: Option<PitcherStats>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct BatterStats {
    // games_played: u32,
    // games_started: u32,
    at_bats: u32,
    runs: u32,
    hits: u32,
    // singles: u32,
    // doubles: u32,
    // triples: u32,
    home_runs: u32,
    runs_batted_in: u32,
    // sacrifice_hits: u32,
    // sacrifice_flys: u32,
    stolen_bases: u32,
    // caught_stealing: u32,
    walks: u32,
    // intentilnal_walks: u32,
    hit_by_pitch: u32,
    // strikeouts: u32,
    // ground_into_double_play: u32,
    total_bases: u32,
    // putouts: u32,
    // assists: u32,
    // errors: u32,
    // extra_base_hits: u32,
    // net_stolen_bases: u32,
    // hitting_for_the_cycle: u32,
    // plate_appearances: u32,
    // grand_slam_home_runs: u32,
    // outfield_assists: u32,
    // double_plays_turned: u32,
    // catcher_interference: u32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PitcherStats {
    // pitching_appearances: u32,
    // games_started: u32,
    innings_pitched: f32,
    wins: u32,
    // losses: u32,
    // complete_games: u32,
    // shutouts: u32,
    saves: u32,
    outs: u32,
    hits: u32,
    // total_batters_faced: u32,
    // runs: u32,
    earned_runs: u32,
    // home_runs: u32,
    walks: u32,
    // intentional_walks: u32,
    hit_batters: u32,
    strikeouts: u32,
    // wild_pitches: u32,
    // balks: u32,
    // stolen_bases_allowed: u32,
    // batters_grounded_into_double_plays: u32,
    // save_chances: u32,
    // holds: u32,
    // total_bases_allowed: u32,
    // pitch_count: u32,
    // singles_allowed: u32,
    // doubles_allowed: u32,
    // triples_allowed: u32,
    // relief_wins: u32,
    // relief_losses: u32,
    // pickoffs: u32,
    // relief_appearances: u32,
    // no_hitters: u32,
    // perfect_games: u32,
    // inherited_runners_scored: u32,
    // quality_starts: u32,
    // blown_saves: u32,
    // net_saves: u32,
    // saves_and_holds: u32,
    // net_saves_and_holds: u32,
}

#[derive(Debug, Default)]
struct FantasyPlayer {
    team: String,
    player: Player,
    fantasy_points: f32,
}
impl FantasyPlayer {
    fn get_stats_string(&self, header_items: &Vec<String>) -> String {
        let bstats = &self.player.batter_stats;
        let pstats = &self.player.pitcher_stats;

        let items: Vec<String> = header_items
            .iter()
            .map(|h| match h.as_str() {
                "Player" => self.player.name.to_owned(),
                "Team" => self.team.to_owned(),
                "FanPts" => self.fantasy_points.to_string(),
                "Pos" => self.player.position.to_owned(),
                "B.AB" => {
                    if let Some(s) = bstats {
                        s.at_bats.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.R" => {
                    if let Some(s) = bstats {
                        s.runs.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.H" => {
                    if let Some(s) = bstats {
                        s.hits.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.HR" => {
                    if let Some(s) = bstats {
                        s.home_runs.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.RBI" => {
                    if let Some(s) = bstats {
                        s.runs_batted_in.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.SB" => {
                    if let Some(s) = bstats {
                        s.stolen_bases.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.BB" => {
                    if let Some(s) = bstats {
                        s.walks.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.HBP" => {
                    if let Some(s) = bstats {
                        s.hit_by_pitch.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "B.TB" => {
                    if let Some(s) = bstats {
                        s.total_bases.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.IP" => {
                    if let Some(s) = pstats {
                        s.innings_pitched.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.W" => {
                    if let Some(s) = pstats {
                        s.wins.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.SV" => {
                    if let Some(s) = pstats {
                        s.saves.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.OUT" => {
                    if let Some(s) = pstats {
                        s.outs.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.H" => {
                    if let Some(s) = pstats {
                        s.hits.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.ER" => {
                    if let Some(s) = pstats {
                        s.earned_runs.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.BB" => {
                    if let Some(s) = pstats {
                        s.walks.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.HBP" => {
                    if let Some(s) = pstats {
                        s.hit_batters.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                "P.K" => {
                    if let Some(s) = pstats {
                        s.strikeouts.to_string()
                    } else {
                        "".to_owned()
                    }
                }
                _ => "".to_owned(),
            })
            .collect();

        items.join(",")
    }
}

#[derive(Debug, Clone)]
struct ApiKeyNotFound;
impl fmt::Display for ApiKeyNotFound {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "both --api_key option and SPORTRADAR_API_KEY env are not set"
        )
    }
}
impl Error for ApiKeyNotFound {}

#[derive(Debug, Clone)]
struct StatFileExists(String);
impl fmt::Display for StatFileExists {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "stats file {} already exists", self.0)
    }
}
impl Error for StatFileExists {}

pub fn show(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let config = get_config(matches)?;

    let filepath = &format!("data/_stats/{}.json", config.date);

    let players = match Path::new(filepath).exists() {
        true => get_players_from_file(filepath)?,
        false => {
            let sr_players = sportradar::get_players(&config)?;
            let ps = convert_players(sr_players)?;
            save_players(filepath, &ps)?;
            ps
        }
    };

    println!("players.length: {}", players.len());

    let league_scoring = scoring::load(&config.league)?;
    let league_roster = roster::load(&config.league)?;
    let fan_players = create_fantasy_players(players, &league_scoring, &league_roster)?;

    print_fantasy_players(fan_players, &config.date, &league_scoring);
    // println!("fantasy players:\n{:#?}", fan_players);

    Ok(())
}

fn print_fantasy_players(players: Vec<FantasyPlayer>, date: &String, s: &scoring::ScoringRule) {
    println!("{}", date);

    let header_items = s.header_items();
    println!("{}", header_items.join(","));

    for p in players.iter() {
        println!("{}", p.get_stats_string(&header_items));
    }
}

fn create_fantasy_players(
    players: Vec<Player>,
    s: &scoring::ScoringRule,
    r: &roster::Roster,
) -> Result<Vec<FantasyPlayer>, Box<dyn Error>> {
    let names: Vec<String> = r.players.iter().map(|p| p.name.to_owned()).collect();
    println!("names: {:#?}", names);

    // TODO: sort by fantasy points
    let players: Vec<FantasyPlayer> = players
        .into_iter()
        .filter(|p| names.iter().find(|&n| *n == p.name).is_some())
        .map(|p| {
            let team = r
                .players
                .iter()
                .find(|rp| rp.name == p.name)
                .map(|rp| rp.team.to_owned())
                .unwrap_or("unknown".to_owned());
            FantasyPlayer {
                team: team,
                fantasy_points: get_fantasy_points(&p, s),
                player: p,
            }
        })
        .collect();

    Ok(sort_by_fantasy_points(players))
}

fn sort_by_fantasy_points(mut players: Vec<FantasyPlayer>) -> Vec<FantasyPlayer> {
    players.sort_by(|a, b| b.fantasy_points.partial_cmp(&a.fantasy_points).unwrap());

    players
}

fn get_fantasy_points(p: &Player, s: &scoring::ScoringRule) -> f32 {
    if let Some(stats) = &p.batter_stats {
        return (stats.at_bats as f32 * s.batter.at_bats)
            + (stats.runs as f32 * s.batter.runs)
            + (stats.hits as f32 * s.batter.hits)
            + (stats.home_runs as f32 * s.batter.home_runs)
            + (stats.runs_batted_in as f32 * s.batter.runs_batted_in)
            + (stats.stolen_bases as f32 * s.batter.stolen_bases)
            + (stats.walks as f32 * s.batter.walks)
            + (stats.hit_by_pitch as f32 * s.batter.hit_by_pitch)
            + (stats.total_bases as f32 * s.batter.total_bases);
    }

    if let Some(stats) = &p.pitcher_stats {
        return (stats.innings_pitched * s.pitcher.innings_pitched)
            + (stats.wins as f32 * s.pitcher.wins)
            + (stats.saves as f32 * s.pitcher.saves)
            + (stats.outs as f32 * s.pitcher.outs)
            + (stats.hits as f32 * s.pitcher.hits)
            + (stats.earned_runs as f32 * s.pitcher.earned_runs)
            + (stats.walks as f32 * s.pitcher.walks)
            + (stats.hit_batters as f32 * s.pitcher.hit_batters)
            + (stats.strikeouts as f32 * s.pitcher.strikeouts);
    }

    return 0.0;
}

fn convert_players(sr_players: Vec<sportradar::Player>) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut players: Vec<Player> = Vec::new();

    for srp in sr_players.iter() {
        let bs = match &srp.statistics.hitting {
            Some(s) if srp.position != "P" => Some(BatterStats {
                at_bats: s.overall.ab,
                runs: s.overall.runs.total,
                hits: s.overall.onbase.h,
                home_runs: s.overall.onbase.hr,
                runs_batted_in: s.overall.rbi,
                stolen_bases: s.overall.steal.stolen,
                walks: s.overall.onbase.bb,
                hit_by_pitch: s.overall.onbase.hbp,
                total_bases: s.overall.onbase.tb,
            }),
            _ => None,
        };
        let ps = match &srp.statistics.pitching {
            Some(s) => Some(PitcherStats {
                innings_pitched: s.overall.ip_2,
                wins: s.overall.games.win,
                saves: s.overall.games.save,
                outs: s.overall.ip_1,
                hits: s.overall.onbase.h,
                earned_runs: s.overall.runs.earned,
                walks: s.overall.onbase.bb,
                hit_batters: s.overall.onbase.hbp,
                strikeouts: s.overall.outs.ktotal,
            }),
            None => None,
        };
        players.push(Player {
            name: format!("{} {}", srp.preferred_name, srp.last_name),
            position: srp.position.to_owned(),
            primary_position: srp.primary_position.to_owned(),
            batter_stats: bs,
            pitcher_stats: ps,
        });
    }

    Ok(players)
}

fn save_players(filepath: &String, players: &Vec<Player>) -> Result<(), Box<dyn Error>> {
    if Path::new(filepath).exists() {
        return Err(Box::new(StatFileExists(filepath.to_owned())));
    }

    fs::create_dir_all("data/_stats")?;
    fs::write(filepath, serde_json::to_string(players)?)?;
    println!("Saved player stats to {} .", filepath);

    Ok(())
}

fn get_players_from_file(filepath: &String) -> Result<Vec<Player>, Box<dyn Error>> {
    println!("Loading players from file {}", filepath);
    let json = fs::read_to_string(filepath)?;
    Ok(serde_json::from_str(&json)?)
}

fn get_config(matches: &ArgMatches) -> Result<Config, Box<dyn Error>> {
    let env_api_key = env::var("SPORTRADAR_API_KEY").unwrap_or("".to_owned());
    let api_key = matches.value_of("api_key").unwrap_or(env_api_key.as_str());
    if api_key == "" {
        return Err(Box::new(ApiKeyNotFound));
    }

    Ok(Config {
        date: matches.value_of("date").unwrap().to_owned(),
        league: matches.value_of("league").unwrap().to_owned(),
        api_key: api_key.to_owned(),
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn convert_players_should_convert_players() {
        use std::fs;
        let json = fs::read_to_string("testdata/batter_stats.json").unwrap();
        let batter: sportradar::Player = serde_json::from_str(&json).unwrap();

        let json = fs::read_to_string("testdata/pitcher_stats.json").unwrap();
        let pitcher: sportradar::Player = serde_json::from_str(&json).unwrap();

        let sr_players = vec![batter, pitcher];
        // println!("sr_players: {:#?}", sr_players);

        let converted = convert_players(sr_players).unwrap();
        // println!("converted: {:#?}", converted);
        assert_eq!(2, converted.len());

        let batter = converted.first().unwrap();
        assert_eq!("Mike Trout", batter.name);
        assert_eq!(true, batter.batter_stats.is_some());
        assert_eq!(false, batter.pitcher_stats.is_some());

        let pitcher = converted.last().unwrap();
        assert_eq!("Trevor Cahill", pitcher.name);
        assert_eq!(false, pitcher.batter_stats.is_some());
        assert_eq!(true, pitcher.pitcher_stats.is_some());
    }

    #[test]
    fn get_players_from_file_should_load_players() {
        let filepath = "testdata/players_converted.json".to_owned();
        let players = get_players_from_file(&filepath).unwrap();

        assert_eq!(3, players.len());

        let batter = players
            .iter()
            .find(|p| p.name == "Tommy La Stella")
            .unwrap();
        println!("batter: {:#?}", batter);
        assert_eq!(true, batter.batter_stats.is_some());
        assert_eq!(false, batter.pitcher_stats.is_some());

        let pitcher = players
            .iter()
            .find(|p| p.name == "Aníbal Sánchez")
            .unwrap();
        println!("pitcher: {:#?}", pitcher);
        assert_eq!(false, pitcher.batter_stats.is_some());
        assert_eq!(true, pitcher.pitcher_stats.is_some());
    }

    fn mock_batter() -> Player {
        Player {
            name: "Trey Mancini".to_owned(),
            position: "OF".to_owned(),
            primary_position: "RF".to_owned(),
            batter_stats: Some(BatterStats {
                at_bats: 3,
                runs: 2,
                hits: 3,
                home_runs: 1,
                runs_batted_in: 2,
                stolen_bases: 0,
                walks: 2,
                hit_by_pitch: 0,
                total_bases: 6,
            }),
            pitcher_stats: None,
        }
    }

    fn mock_pitcher() -> Player {
        Player {
            name: "Blake Snell".to_owned(),
            position: "P".to_owned(),
            primary_position: "SP".to_owned(),
            batter_stats: None,
            pitcher_stats: Some(PitcherStats {
                innings_pitched: 6.0,
                wins: 1,
                saves: 0,
                outs: 18,
                hits: 6,
                earned_runs: 1,
                walks: 0,
                hit_batters: 0,
                strikeouts: 11,
            }),
        }
    }

    fn mock_scoring_rule() -> scoring::ScoringRule {
        scoring::load(&"sample".to_owned()).unwrap()
    }

    #[test]
    fn get_fantasy_points_should_return_fantasy_points() {
        let batter = mock_batter();
        let pitcher = mock_pitcher();

        let sr = mock_scoring_rule();

        assert_eq!(13.5, get_fantasy_points(&batter, &sr));
        assert_eq!(32.5, get_fantasy_points(&pitcher, &sr));
    }

    #[test]
    fn fantasy_player_get_stats_string_should_return_string() {
        let s = mock_scoring_rule();
        let header_items = s.header_items();

        let batter = mock_batter();
        let fp = FantasyPlayer {
            team: "Avengers".to_owned(),
            fantasy_points: get_fantasy_points(&batter, &s),
            player: batter,
        };

        assert_eq!(
            "Trey Mancini,Avengers,13.5,OF,2,3,1,2,0,,,,,".to_owned(),
            fp.get_stats_string(&header_items)
        );

        let pitcher = mock_pitcher();
        let fp = FantasyPlayer {
            team: "Avengers".to_owned(),
            fantasy_points: get_fantasy_points(&pitcher, &s),
            player: pitcher,
        };

        assert_eq!(
            "Blake Snell,Avengers,32.5,P,,,,,,6,1,0,1,11".to_owned(),
            fp.get_stats_string(&header_items)
        );
    }

    #[test]
    fn sort_by_fantasy_points_should_sort_players_by_fan_pts_descending() {
        let mut p1: FantasyPlayer = Default::default();
        p1.fantasy_points = 1.0;
        let mut p2: FantasyPlayer = Default::default();
        p2.fantasy_points = 10.0;
        let mut p3: FantasyPlayer = Default::default();
        p3.fantasy_points = -2.0;

        let players = vec![p1, p2, p3];

        let sorted = sort_by_fantasy_points(players);
        let points: Vec<f32> = sorted.iter().map(|p| p.fantasy_points).collect();

        assert_eq!(vec![10.0, 1.0, -2.0], points);
    }
}
