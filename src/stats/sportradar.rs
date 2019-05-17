use crate::stats::schedule;
use crate::stats::Config;
use crate::utils;
use serde::Deserialize;
use std::error::Error;
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct Summary {
    game: Game,
}

#[derive(Deserialize, Debug)]
struct Game {
    id: String,
    home: Team,
    away: Team,
}

#[derive(Deserialize, Debug)]
struct Team {
    abbr: String,
    players: Vec<Player>,
}

#[derive(Deserialize, Debug, Default)]
pub struct Player {
    pub preferred_name: String,
    pub last_name: String,
    pub status: String,
    pub position: String,
    pub primary_position: String,
    pub statistics: Stats,
}

#[derive(Deserialize, Debug, Default)]
pub struct Stats {
    pub hitting: Option<HittingStats>,
    pub pitching: Option<PitchingStats>,
}

#[derive(Deserialize, Debug, Default)]
pub struct HittingStats {
    pub overall: HittingOverallStats,
}

#[derive(Deserialize, Debug, Default)]
pub struct PitchingStats {
    pub overall: PitchingOverallStats,
}

#[derive(Deserialize, Debug, Default)]
pub struct HittingOverallStats {
    pub ab: u32,
    pub rbi: u32,
    pub onbase: HitterOnBaseStats,
    pub runs: HitterRunStats,
    pub outs: OutStats,
    pub steal: StealStats,
}

#[derive(Deserialize, Debug, Default)]
pub struct HitterOnBaseStats {
    pub s: u32,
    pub d: u32,
    pub t: u32,
    pub hr: u32,
    pub tb: u32,
    pub bb: u32,
    pub ibb: u32,
    pub hbp: u32,
    pub fc: u32,
    pub roe: u32,
    pub h: u32,
    pub cycle: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct HitterRunStats {
    pub total: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct OutStats {
    pub po: u32,
    pub fo: u32,
    pub fidp: u32,
    pub lo: u32,
    pub lidp: u32,
    pub go: u32,
    pub gidp: u32,
    pub klook: u32,
    pub kswing: u32,
    pub ktotal: u32,
    pub sacfly: u32,
    pub sachit: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct StealStats {
    pub caught: u32,
    pub stolen: u32,
    pub pickoff: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct PitchingOverallStats {
    pub lob: u32,
    pub pitch_count: u32,
    pub wp: u32,
    pub bk: u32,
    pub ip_1: u32, // outs
    pub ip_2: f32, // innings pitched (ex: 1.0, 1.1, 1.2)
    pub bf: u32,
    pub onbase: PitcherOnBaseStats,
    pub runs: PitcherRunStats,
    pub outs: OutStats,
    pub steal: StealStats,
    pub games: PitcherGameStats,
}

#[derive(Deserialize, Debug, Default)]
pub struct PitcherOnBaseStats {
    pub s: u32,
    pub d: u32,
    pub t: u32,
    pub hr: u32,
    pub tb: u32,
    pub bb: u32,
    pub ibb: u32,
    pub hbp: u32,
    pub fc: u32,
    pub roe: u32,
    pub h: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct PitcherRunStats {
    pub total: u32,
    pub unearned: u32,
    pub earned: u32,
}

#[derive(Deserialize, Debug, Default)]
pub struct PitcherGameStats {
    pub start: u32,
    pub play: u32,
    pub finish: u32,
    pub svo: u32,
    pub qstart: u32,
    pub shutout: u32,
    pub complete: u32,
    pub win: u32,
    pub loss: u32,
    pub save: u32,
    pub hold: u32,
    pub blown_save: u32,
    pub team_win: u32,
    pub team_loss: u32,
}

pub fn get_players(config: &Config) -> Result<Vec<Player>, Box<dyn Error>> {
    let game_ids = schedule::get_game_ids(config)?;
    // println!("game_ids: {:#?}", game_ids);

    let mut result: Vec<Player> = Vec::new();

    for (i, id) in game_ids.iter().enumerate() {
        thread::sleep(Duration::from_millis(1050));
        let url = get_game_summary_url(config, id);
        // println!("game summary api url: {}", url);

        let json = utils::get_json_res(&url)?;
        match get_players_from_string(json) {
            Ok(p) => {
                let mut players = p.into_iter().filter(|p| p.status == "A").collect();
                result.append(&mut players);
            }
            Err(e) => {
                println!("Fetch failed: {}", e);
            }
        }

        println!("game summary fetched: {}/{}", i + 1, game_ids.len());
    }

    Ok(result)
}

fn get_players_from_string(json: String) -> Result<Vec<Player>, Box<dyn Error>> {
    let mut summary: Summary = serde_json::from_str(&json)?;
    let mut players: Vec<Player> = Vec::new();

    players.append(&mut summary.game.home.players);
    players.append(&mut summary.game.away.players);

    Ok(players)
}

fn get_game_summary_url(config: &Config, game_id: &str) -> String {
    format!(
        "https://api.sportradar.us/mlb-t6/games/{}/summary.json?api_key={}",
        game_id, config.api_key
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_players_from_string_should_return_players() {
        use std::fs;
        let json = fs::read_to_string("testdata/summary.json").unwrap();
        let players = get_players_from_string(json).unwrap();
        assert_eq!(27, players.len());

        let hitter = players.iter().find(|p| p.last_name == "Trout").unwrap();
        // println!("hitter: {:#?}", hitter);
        assert_eq!(true, hitter.statistics.hitting.is_some());
        assert_eq!(true, hitter.statistics.pitching.is_none());

        let pitcher = players.iter().find(|p| p.last_name == "Cahill").unwrap();
        // println!("pitcher: {:#?}", pitcher);
        assert_eq!(true, pitcher.statistics.pitching.is_some());
        assert_eq!(true, pitcher.statistics.hitting.is_none());
    }
}
