use crate::league::{roster, scoring};
use crate::utils;
use clap::ArgMatches;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::Add;
use std::path::Path;
use std::rc::Rc;

mod output;
mod schedule;
mod sportradar;

pub struct Config<'a> {
    date: Cow<'a, str>,
    range: Cow<'a, str>,
    league: Cow<'a, str>,
    api_key: Cow<'a, str>,
    format: Cow<'a, str>,
    show_all: bool,
    top_n: usize,
}
impl<'a> Config<'a> {
    pub fn new<S>(
        date: S,
        range: S,
        league: S,
        api_key: S,
        format: S,
        show_all: bool,
        top_n: usize,
    ) -> Config<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Config {
            date: date.into(),
            range: range.into(),
            league: league.into(),
            api_key: api_key.into(),
            format: format.into(),
            show_all,
            top_n,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Player<'a> {
    name: Cow<'a, str>,
    position: Cow<'a, str>,
    primary_position: Cow<'a, str>,
    batter_stats: Option<BatterStats>,
    pitcher_stats: Option<PitcherStats>,
}

#[allow(dead_code)]
impl<'a> Player<'a> {
    fn new_batter<S>(name: S, position: S, primary_position: S, stats: BatterStats) -> Player<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Player {
            name: name.into(),
            position: position.into(),
            primary_position: primary_position.into(),
            batter_stats: Some(stats),
            pitcher_stats: None,
        }
    }

    fn new_pitcher<S>(name: S, position: S, primary_position: S, stats: PitcherStats) -> Player<'a>
    where
        S: Into<Cow<'a, str>>,
    {
        Player {
            name: name.into(),
            position: position.into(),
            primary_position: primary_position.into(),
            batter_stats: None,
            pitcher_stats: Some(stats),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct BatterStats {
    at_bats: u32,
    runs: u32,
    hits: u32,
    singles: u32,
    doubles: u32,
    triples: u32,
    home_runs: u32,
    runs_batted_in: u32,
    sacrifice_hits: u32,
    stolen_bases: u32,
    caught_stealing: u32,
    walks: u32,
    intentional_walks: u32,
    hit_by_pitch: u32,
    strikeouts: u32,
    ground_into_double_play: u32,
    total_bases: u32,
}
impl Add for BatterStats {
    type Output = BatterStats;

    fn add(self, other: BatterStats) -> BatterStats {
        BatterStats {
            at_bats: self.at_bats + other.at_bats,
            runs: self.runs + other.runs,
            hits: self.hits + other.hits,
            singles: self.singles + other.singles,
            doubles: self.doubles + other.doubles,
            triples: self.triples + other.triples,
            home_runs: self.home_runs + other.home_runs,
            runs_batted_in: self.runs_batted_in + other.runs_batted_in,
            sacrifice_hits: self.sacrifice_hits + other.sacrifice_hits,
            stolen_bases: self.stolen_bases + other.stolen_bases,
            caught_stealing: self.caught_stealing + other.caught_stealing,
            walks: self.walks + other.walks,
            intentional_walks: self.intentional_walks + other.intentional_walks,
            hit_by_pitch: self.hit_by_pitch + other.hit_by_pitch,
            strikeouts: self.strikeouts + other.strikeouts,
            ground_into_double_play: self.ground_into_double_play + other.ground_into_double_play,
            total_bases: self.total_bases + other.total_bases,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct PitcherStats {
    innings_pitched: f32,
    wins: u32,
    losses: u32,
    complete_games: u32,
    shutouts: u32,
    saves: u32,
    outs: u32,
    hits: u32,
    earned_runs: u32,
    home_runs: u32,
    walks: u32,
    intentional_walks: u32,
    hit_batters: u32,
    strikeouts: u32,
    stolen_bases_allowed: u32,
    batters_grounded_into_double_plays: u32,
    total_bases_allowed: u32,
}
impl Add for PitcherStats {
    type Output = PitcherStats;

    fn add(self, other: PitcherStats) -> PitcherStats {
        let mut ip = self.innings_pitched + other.innings_pitched;
        let dp = ip % 1.0;
        if dp > 0.2 {
            ip = ((ip + 1.0 - 0.3) * 10.0).round() / 10.0;
        } else {
            ip = (ip * 10.0).round() / 10.0;
        }

        PitcherStats {
            innings_pitched: ip,
            wins: self.wins + other.wins,
            losses: self.losses + other.losses,
            complete_games: self.complete_games + other.complete_games,
            shutouts: self.shutouts + other.shutouts,
            saves: self.saves + other.saves,
            outs: self.outs + other.outs,
            hits: self.hits + other.hits,
            earned_runs: self.earned_runs + other.earned_runs,
            home_runs: self.home_runs + other.home_runs,
            walks: self.walks + other.walks,
            intentional_walks: self.intentional_walks + other.intentional_walks,
            hit_batters: self.hit_batters + other.hit_batters,
            strikeouts: self.strikeouts + other.strikeouts,
            stolen_bases_allowed: self.stolen_bases_allowed + other.stolen_bases_allowed,
            batters_grounded_into_double_plays: self.batters_grounded_into_double_plays
                + other.batters_grounded_into_double_plays,
            total_bases_allowed: self.total_bases_allowed + other.total_bases_allowed,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct FantasyPlayer<'a> {
    team: Rc<Cow<'a, str>>,
    player: Player<'a>,
    fantasy_points: f32,
}
impl<'a> FantasyPlayer<'a> {
    fn add_stats(&mut self, other: FantasyPlayer<'a>) {
        let player = other.player;

        if let Some(ps) = self.player.pitcher_stats.clone() {
            let other_pitcher = player.clone();
            match other_pitcher.pitcher_stats {
                Some(stats) => {
                    self.player.pitcher_stats = Some(ps + stats)
                }
                None => warn!(
                    "{}: self - position: {}, pither_stats: {}, other - position: {}, pither_stats: {}", self.player.name,
                    self.player.primary_position, self.player.pitcher_stats.is_some(),
                    other_pitcher.primary_position,
                    other_pitcher.pitcher_stats.is_some(),
                ),
            }
        } else if let Some(bs) = self.player.batter_stats.clone() {
            let other_batter = player.clone();
            match other_batter.batter_stats {
                Some(stats) => self.player.batter_stats = Some(bs + stats),
                None => warn!(
                    "{}: self - position: {}, batter_stats: {}, other - position: {}, batter_stats: {}", self.player.name,
                    self.player.primary_position, self.player.batter_stats.is_some(),
                    other_batter.primary_position,
                    other_batter.batter_stats.is_some(),
                ),
            }
        }

        self.fantasy_points += other.fantasy_points;
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
    let env_api_key = get_env_api_key();
    let config = get_config(matches, &env_api_key)?;

    let dates = utils::date_strs(&config.date, &config.range);

    let players: Vec<_> = dates
        .into_iter()
        .flat_map(|d| {
            let f = Box::leak(
                format!("{}/.mlbh2h/stats/{}.json", utils::get_home_dir(), d).into_boxed_str(),
            );
            let players = if Path::new(f).exists() {
                get_players_from_file(f).expect("error getting players from file")
            } else {
                let sr_players = sportradar::get_players(&config, &d[..])
                    .expect("error getting players via sportradar API");
                let ps = convert_players(sr_players).expect("error converting players");
                save_players(f, &ps).expect("error saving players");
                ps
            };
            players
        })
        .collect();

    let league = config.league.to_owned().into();
    let league_scoring = scoring::load(&league)?;
    let league_roster = roster::load(&league)?;
    let fan_players =
        create_fantasy_players(&players, &league_scoring, &league_roster, config.show_all)?;

    println!();
    print_fantasy_players(fan_players, &config, &league_scoring);

    Ok(())
}

fn print_fantasy_players(players: Vec<FantasyPlayer>, config: &Config, s: &scoring::ScoringRule) {
    let date = &config.date;
    println!("{} ({})", date, &config.range);

    let header_items = s.get_header_items();

    let is_csv = config.format == Cow::Borrowed("csv");

    if config.top_n > 0 {
        let mut batters: Vec<FantasyPlayer> = vec![];
        let mut pitchers: Vec<FantasyPlayer> = vec![];
        for p in players.into_iter() {
            if p.player.batter_stats.is_some() && batters.len() < config.top_n {
                batters.push(p);
            } else if p.player.pitcher_stats.is_some() && pitchers.len() < config.top_n {
                pitchers.push(p);
            }

            if batters.len() >= config.top_n && pitchers.len() >= config.top_n {
                break;
            }
        }

        let batter_header_items = s.get_header_items_for_batter();
        let pitcher_header_items = s.get_header_items_for_pitcher();

        println!("\n## Top {} Batters ##", config.top_n);
        println!(
            "{}",
            output::get_header_string(&batter_header_items, is_csv)
        );
        for p in batters.iter() {
            println!(
                "{}",
                output::get_player_stats_string(p, &batter_header_items, is_csv)
            );
        }

        println!("\n## Top {} Pitchers ##", config.top_n);
        println!(
            "{}",
            output::get_header_string(&pitcher_header_items, is_csv)
        );
        for p in pitchers.iter() {
            println!(
                "{}",
                output::get_player_stats_string(p, &pitcher_header_items, is_csv)
            );
        }
    } else {
        println!("{}", output::get_header_string(&header_items, is_csv));
        for p in players.iter() {
            println!(
                "{}",
                output::get_player_stats_string(p, &header_items, is_csv)
            );
        }
    }
}

fn create_fantasy_players<'a>(
    players: &'a [Player],
    s: &scoring::ScoringRule,
    r: &roster::Roster<'a>,
    show_all: bool,
) -> Result<Vec<FantasyPlayer<'a>>, Box<dyn Error>> {
    let names: Vec<Cow<'a, str>> = r.players.iter().map(|p| p.name.to_owned()).collect();

    let team_fa = Rc::new(Cow::Borrowed("<FA>"));
    let players: Vec<FantasyPlayer> = players
        .iter()
        .filter(|p| {
            if show_all {
                true
            } else {
                names
                    .iter()
                    .any(|n| n.to_lowercase() == p.name.to_lowercase())
            }
        })
        .map(|p| {
            let team = r
                .players
                .iter()
                .find(|rp| rp.name.to_lowercase() == p.name.to_lowercase())
                .map(|rp| Rc::clone(&rp.team))
                .unwrap_or_else(|| Rc::clone(&team_fa));
            FantasyPlayer {
                team,
                fantasy_points: get_fantasy_points(&p, s),
                player: p.clone(),
            }
        })
        .collect();

    let players = merge_same_players_stats(players);

    Ok(sort_by_fantasy_points(players))
}

fn merge_same_players_stats<'a>(players: Vec<FantasyPlayer<'a>>) -> Vec<FantasyPlayer<'a>> {
    let mut map = HashMap::<String, FantasyPlayer<'a>>::new();
    let map = players.into_iter().fold(&mut map, |m, p| {
        let name = p.player.name.to_string();
        m.entry(name)
            .and_modify(|x| {
                // TODO: remove data cloning (p.clone())
                x.add_stats(p.clone());
            })
            .or_insert(p);
        m
    });

    // TODO: remove data cloning
    map.values().cloned().collect::<Vec<_>>()
}

fn sort_by_fantasy_points<'a>(mut players: Vec<FantasyPlayer<'a>>) -> Vec<FantasyPlayer<'a>> {
    players.sort_by(|a, b| b.fantasy_points.partial_cmp(&a.fantasy_points).unwrap());

    players
}

pub fn get_fantasy_points(p: &Player, s: &scoring::ScoringRule) -> f32 {
    if let Some(stats) = &p.batter_stats {
        return (stats.at_bats as f32 * s.batter.at_bats)
            + (stats.runs as f32 * s.batter.runs)
            + (stats.hits as f32 * s.batter.hits)
            + (stats.singles as f32 * s.batter.singles)
            + (stats.doubles as f32 * s.batter.doubles)
            + (stats.triples as f32 * s.batter.triples)
            + (stats.home_runs as f32 * s.batter.home_runs)
            + (stats.runs_batted_in as f32 * s.batter.runs_batted_in)
            + (stats.sacrifice_hits as f32 * s.batter.sacrifice_hits)
            + (stats.stolen_bases as f32 * s.batter.stolen_bases)
            + (stats.caught_stealing as f32 * s.batter.caught_stealing)
            + (stats.walks as f32 * s.batter.walks)
            + (stats.intentional_walks as f32 * s.batter.intentional_walks)
            + (stats.hit_by_pitch as f32 * s.batter.hit_by_pitch)
            + (stats.strikeouts as f32 * s.batter.strikeouts)
            + (stats.ground_into_double_play as f32 * s.batter.ground_into_double_play)
            + (stats.total_bases as f32 * s.batter.total_bases);
    }

    if let Some(stats) = &p.pitcher_stats {
        return inning_score(stats.innings_pitched, s.pitcher.innings_pitched)
            + (stats.wins as f32 * s.pitcher.wins)
            + (stats.losses as f32 * s.pitcher.losses)
            + (stats.complete_games as f32 * s.pitcher.complete_games)
            + (stats.shutouts as f32 * s.pitcher.shutouts)
            + (stats.saves as f32 * s.pitcher.saves)
            + (stats.outs as f32 * s.pitcher.outs)
            + (stats.hits as f32 * s.pitcher.hits)
            + (stats.earned_runs as f32 * s.pitcher.earned_runs)
            + (stats.home_runs as f32 * s.pitcher.home_runs)
            + (stats.walks as f32 * s.pitcher.walks)
            + (stats.intentional_walks as f32 * s.pitcher.intentional_walks)
            + (stats.hit_batters as f32 * s.pitcher.hit_batters)
            + (stats.strikeouts as f32 * s.pitcher.strikeouts)
            + (stats.stolen_bases_allowed as f32 * s.pitcher.stolen_bases_allowed)
            + (stats.batters_grounded_into_double_plays as f32
                * s.pitcher.batters_grounded_into_double_plays)
            + (stats.total_bases_allowed as f32 * s.pitcher.total_bases_allowed);
    }

    0.0
}

fn inning_score(inning_pitched: f32, score: f32) -> f32 {
    let quotient = ((inning_pitched * 10.0) as i32 / 10) as f32;
    let remainder = (inning_pitched * 10.0 % 10.0) as i32;

    match remainder {
        1 => (quotient * score) + (score / 3.0),
        2 => (quotient * score) + (score * 2.0 / 3.0),
        _ => (quotient * score),
    }
}

fn convert_players<'a>(
    sr_players: Vec<sportradar::Player>,
) -> Result<Vec<Player<'a>>, Box<dyn Error>> {
    let mut players: Vec<Player> = Vec::new();

    for srp in sr_players.iter() {
        let bs = match &srp.statistics.hitting {
            Some(s) if srp.position != "P" => Some(BatterStats {
                at_bats: s.overall.ab,
                runs: s.overall.runs.total,
                hits: s.overall.onbase.h,
                singles: s.overall.onbase.s,
                doubles: s.overall.onbase.d,
                triples: s.overall.onbase.t,
                home_runs: s.overall.onbase.hr,
                runs_batted_in: s.overall.rbi,
                sacrifice_hits: s.overall.outs.sachit,
                stolen_bases: s.overall.steal.stolen,
                caught_stealing: s.overall.steal.caught,
                walks: s.overall.onbase.bb,
                intentional_walks: s.overall.onbase.ibb,
                hit_by_pitch: s.overall.onbase.hbp,
                strikeouts: s.overall.outs.ktotal,
                ground_into_double_play: s.overall.outs.gidp,
                total_bases: s.overall.onbase.tb,
            }),
            _ => None,
        };
        let ps = match &srp.statistics.pitching {
            Some(s) => Some(PitcherStats {
                innings_pitched: s.overall.ip_2,
                wins: s.overall.games.win,
                losses: s.overall.games.loss,
                complete_games: s.overall.games.complete,
                shutouts: s.overall.games.shutout,
                saves: s.overall.games.save,
                outs: s.overall.ip_1,
                hits: s.overall.onbase.h,
                earned_runs: s.overall.runs.earned,
                home_runs: s.overall.onbase.hr,
                walks: s.overall.onbase.bb,
                intentional_walks: s.overall.onbase.ibb,
                hit_batters: s.overall.onbase.hbp,
                strikeouts: s.overall.outs.ktotal,
                stolen_bases_allowed: s.overall.steal.stolen,
                batters_grounded_into_double_plays: s.overall.outs.gidp,
                total_bases_allowed: s.overall.onbase.tb,
            }),
            None => None,
        };

        players.push(Player {
            name: Cow::Owned(format!("{} {}", srp.preferred_name, srp.last_name)),
            position: Cow::Owned(srp.position.to_owned()),
            primary_position: Cow::Owned(srp.primary_position.to_owned()),
            batter_stats: bs,
            pitcher_stats: ps,
        });
    }

    Ok(players)
}

fn save_players(filepath: &str, players: &[Player]) -> Result<(), Box<dyn Error>> {
    if Path::new(filepath).exists() {
        return Err(Box::new(StatFileExists(filepath.to_string())));
    }

    fs::create_dir_all(format!("{}/.mlbh2h/stats", utils::get_home_dir()))?;
    fs::write(filepath, serde_json::to_string(players)?)?;
    info!("Saved player stats to {} .", filepath);

    Ok(())
}

fn get_players_from_file(filepath: &str) -> Result<Vec<Player>, Box<dyn Error>> {
    info!("Loading players from file {}", filepath);
    let json = fs::read_to_string(filepath)?;
    Ok(serde_json::from_str(&json)?)
}

fn get_env_api_key() -> String {
    env::var("SPORTRADAR_API_KEY").unwrap_or_default()
}

fn get_config<'a>(
    matches: &'a ArgMatches,
    env_api_key: &'a str,
) -> Result<Config<'a>, Box<dyn Error>> {
    let api_key = matches.value_of("api_key").unwrap_or(env_api_key);
    if api_key == "" {
        return Err(Box::new(ApiKeyNotFound));
    }

    Ok(Config::new(
        matches.value_of("date").unwrap(),
        matches.value_of("range").unwrap(),
        matches.value_of("league").unwrap(),
        api_key,
        matches.value_of("format").unwrap(),
        matches.occurrences_of("all") > 0,
        (matches.occurrences_of("topn") * 10) as usize,
    ))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::assert_eq_f32;

    fn mock_batter<'a>() -> Player<'a> {
        Player::new_batter(
            "Trey Mancini",
            "OF",
            "RF",
            BatterStats {
                at_bats: 3,
                runs: 2,
                hits: 3,
                singles: 2,
                doubles: 0,
                triples: 0,
                home_runs: 1,
                runs_batted_in: 2,
                sacrifice_hits: 0,
                stolen_bases: 0,
                caught_stealing: 0,
                walks: 2,
                intentional_walks: 0,
                hit_by_pitch: 0,
                strikeouts: 0,
                ground_into_double_play: 0,
                total_bases: 6,
            },
        )
    }

    fn mock_fa_batter<'a>() -> Player<'a> {
        Player::new_batter(
            "Andrew McCutchen",
            "OF",
            "LF",
            BatterStats {
                at_bats: 3,
                runs: 0,
                hits: 1,
                singles: 1,
                doubles: 0,
                triples: 0,
                home_runs: 0,
                runs_batted_in: 0,
                sacrifice_hits: 0,
                stolen_bases: 0,
                caught_stealing: 0,
                walks: 1,
                intentional_walks: 0,
                hit_by_pitch: 0,
                strikeouts: 0,
                ground_into_double_play: 0,
                total_bases: 1,
            },
        )
    }

    fn mock_pitcher<'a>() -> Player<'a> {
        Player::new_pitcher(
            "Blake Snell",
            "P",
            "SP",
            PitcherStats {
                innings_pitched: 6.0,
                wins: 1,
                losses: 0,
                complete_games: 0,
                shutouts: 0,
                saves: 0,
                outs: 18,
                hits: 6,
                earned_runs: 1,
                home_runs: 1,
                walks: 0,
                intentional_walks: 0,
                hit_batters: 0,
                strikeouts: 11,
                stolen_bases_allowed: 1,
                batters_grounded_into_double_plays: 1,
                total_bases_allowed: 10,
            },
        )
    }

    #[test]
    fn convert_players_should_convert_players() {
        use std::fs;
        let json = fs::read_to_string("testdata/batter_stats.json").unwrap();
        let batter: sportradar::Player = serde_json::from_str(&json).unwrap();

        let json = fs::read_to_string("testdata/pitcher_stats.json").unwrap();
        let pitcher: sportradar::Player = serde_json::from_str(&json).unwrap();

        let sr_players = vec![batter, pitcher];

        let converted = convert_players(sr_players).unwrap();
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
        let filepath = "testdata/players_converted.json".to_string();
        let players = get_players_from_file(&filepath).unwrap();

        assert_eq!(3, players.len());

        let batter = players.iter().find(|p| p.name == "Trey Mancini").unwrap();
        assert_eq!(true, batter.batter_stats.is_some());
        assert_eq!(false, batter.pitcher_stats.is_some());

        let pitcher = players.iter().find(|p| p.name == "Blake Snell").unwrap();
        assert_eq!(false, pitcher.batter_stats.is_some());
        assert_eq!(true, pitcher.pitcher_stats.is_some());
    }

    #[test]
    fn get_fantasy_points_should_return_fantasy_points() {
        use crate::league::scoring::sample_scoring_rule;
        let sr = sample_scoring_rule();

        let batter = mock_batter();
        let pitcher = mock_pitcher();

        assert_eq_f32(13.5, get_fantasy_points(&batter, &sr));
        assert_eq_f32(32.5, get_fantasy_points(&pitcher, &sr));
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

    #[test]
    fn inning_score_should_return_score() {
        assert_eq_f32(9.0, inning_score(3.0, 3.0));
        assert_eq_f32(10.0, inning_score(3.1, 3.0));
        assert_eq_f32(11.0, inning_score(3.2, 3.0));
    }

    #[test]
    fn create_fantasy_players_should_apply_to_names_case_insensitive() {
        use crate::league::scoring::sample_scoring_rule;
        let sr = sample_scoring_rule();

        use crate::league::roster::sample_roster;
        let r = sample_roster();

        let mut batter = mock_batter();
        batter.name = Cow::Owned(batter.name.to_lowercase());

        let players = vec![batter];

        let f_players = create_fantasy_players(&players, &sr, &r, false).unwrap();

        assert_eq!(1, f_players.len());
    }

    #[test]
    fn create_fantasy_players_should_show_fa_players_when_show_all_is_true() {
        use crate::league::scoring::sample_scoring_rule;
        let sr = sample_scoring_rule();

        use crate::league::roster::sample_roster;
        let r = sample_roster();

        let players = vec![mock_batter(), mock_fa_batter()];

        let f_players = create_fantasy_players(&players, &sr, &r, false).unwrap();

        assert_eq!(1, f_players.len());

        let f_players = create_fantasy_players(&players, &sr, &r, true).unwrap();

        assert_eq!(2, f_players.len());
    }

    #[test]
    fn add_stats_for_fantasy_player_should_sum_stats() {
        let team = Rc::new(Cow::Borrowed("A-Team"));
        let p = mock_batter();

        let mut fp1 = FantasyPlayer {
            team: team.clone(),
            player: mock_batter(),
            ..Default::default()
        };
        let fp2 = FantasyPlayer {
            team: team.clone(),
            player: mock_batter(),
            ..Default::default()
        };
        fp1.add_stats(fp2);

        assert_eq!(
            p.batter_stats.unwrap().at_bats * 2,
            fp1.player.batter_stats.unwrap().at_bats
        );

        assert_eq!(true, fp1.player.pitcher_stats.is_none());
    }

    #[test]
    fn merge_same_players_stats_should_remove_dups() {
        let fantasy_players = vec![
            FantasyPlayer {
                player: mock_batter(),
                ..Default::default()
            },
            FantasyPlayer {
                player: mock_pitcher(),
                ..Default::default()
            },
            FantasyPlayer {
                player: mock_batter(),
                ..Default::default()
            },
        ];

        let merged = merge_same_players_stats(fantasy_players);

        assert_eq!(2, merged.len());
        assert_eq!(
            mock_batter().batter_stats.unwrap().at_bats * 2,
            merged
                .into_iter()
                .find(|fp| fp.player.batter_stats.is_some())
                .unwrap()
                .player
                .batter_stats
                .unwrap()
                .at_bats
        );
    }
}
