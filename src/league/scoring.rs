use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BatterScoringRule {
    // games_played: f32,
    // games_started: f32,
    at_bats: f32,
    runs: f32,
    hits: f32,
    // singles: f32,
    // doubles: f32,
    // triples: f32,
    home_runs: f32,
    runs_batted_in: f32,
    // sacrifice_hits: f32,
    // sacrifice_flys: f32,
    stolen_bases: f32,
    // caught_stealing: f32,
    // walks: f32,
    // intentilnal_walks: f32,
    hit_by_pitch: f32,
    // strikeouts: f32,
    // ground_into_double_play: f32,
    total_bases: f32,
    // putouts: f32,
    // assists: f32,
    // errors: f32,
    // extra_base_hits: f32,
    // net_stolen_bases: f32,
    // hitting_for_the_cycle: f32,
    // plate_appearances: f32,
    // grand_slam_home_runs: f32,
    // outfield_assists: f32,
    // double_plays_turned: f32,
    // catcher_interference: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PitcherScoringRule {
    // pitching_appearances: f32,
    // games_started: f32,
    innings_pitched: f32,
    wins: f32,
    // losses: f32,
    // complete_games: f32,
    // shutouts: f32,
    saves: f32,
    outs: f32,
    hits: f32,
    // total_batters_faced: f32,
    // runs: f32,
    earned_runs: f32,
    // home_runs: f32,
    // walks: f32,
    // intentional_walks: f32,
    hit_batters: f32,
    strikeouts: f32,
    // wild_pitches: f32,
    // balks: f32,
    // stolen_bases_allowed: f32,
    // batters_grounded_into_double_plays: f32,
    // save_chances: f32,
    // holds: f32,
    // total_bases_allowed: f32,
    // pitch_count: f32,
    // singles_allowed: f32,
    // doubles_allowed: f32,
    // triples_allowed: f32,
    // relief_wins: f32,
    // relief_losses: f32,
    // pickoffs: f32,
    // relief_appearances: f32,
    // no_hitters: f32,
    // perfect_games: f32,
    // inherited_runners_scored: f32,
    // quality_starts: f32,
    // blown_saves: f32,
    // net_saves: f32,
    // saves_and_holds: f32,
    // net_saves_and_holds: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScoringRule {
    batter: BatterScoringRule,
    pitcher: PitcherScoringRule,
}

pub fn add(dir: &str) -> Result<ScoringRule, Box<dyn Error>> {
    let filepath = &format!("{}/scoring.json", dir);

    let rule = get_scorings_from_stdin()?;

    fs::write(filepath, serde_json::to_string(&rule)?)?;
    println!("Saved scoring rule to {}.", filepath);

    Ok(rule)
}

fn get_scorings_from_stdin() -> Result<ScoringRule, Box<dyn Error>> {
    let mut rule: ScoringRule = Default::default();

    // rule.batter.games_played = get_stdin("batter.games_played")?;
    // rule.batter.games_started = get_stdin("batter.games_started")?;
    rule.batter.at_bats = get_stdin("batter.at_bats")?;
    rule.batter.runs = get_stdin("batter.runs")?;
    rule.batter.hits = get_stdin("batter.hits")?;
    // rule.batter.singles = get_stdin("batter.singles")?;
    // rule.batter.doubles = get_stdin("batter.doubles")?;
    // rule.batter.triples = get_stdin("batter.triples")?;
    rule.batter.home_runs = get_stdin("batter.home_runs")?;
    rule.batter.runs_batted_in = get_stdin("batter.runs_batted_in")?;
    // rule.batter.sacrifice_hits = get_stdin("batter.sacrifice_hits")?;
    // rule.batter.sacrifice_flys = get_stdin("batter.sacrifice_flys")?;
    rule.batter.stolen_bases = get_stdin("batter.stolen_bases")?;
    // rule.batter.caught_stealing = get_stdin("batter.caught_stealing")?;
    // rule.batter.walks = get_stdin("batter.walks")?;
    // rule.batter.intentilnal_walks = get_stdin("batter.intentilnal_walks")?;
    rule.batter.hit_by_pitch = get_stdin("batter.hit_by_pitch")?;
    // rule.batter.strikeouts = get_stdin("batter.strikeouts")?;
    // rule.batter.ground_into_double_play = get_stdin("batter.ground_into_double_play")?;
    rule.batter.total_bases = get_stdin("batter.total_bases")?;
    // rule.batter.putouts = get_stdin("batter.putouts")?;
    // rule.batter.assists = get_stdin("batter.assists")?;
    // rule.batter.errors = get_stdin("batter.errors")?;
    // rule.batter.extra_base_hits = get_stdin("batter.extra_base_hits")?;
    // rule.batter.net_stolen_bases = get_stdin("batter.net_stolen_bases")?;
    // rule.batter.hitting_for_the_cycle = get_stdin("batter.hitting_for_the_cycle")?;
    // rule.batter.plate_appearances = get_stdin("batter.plate_appearances")?;
    // rule.batter.grand_slam_home_runs = get_stdin("batter.grand_slam_home_runs")?;
    // rule.batter.outfield_assists = get_stdin("batter.outfield_assists")?;
    // rule.batter.double_plays_turned = get_stdin("batter.double_plays_turned")?;
    // rule.batter.catcher_interference = get_stdin("batter.catcher_interference")?;

    // rule.pitcher.pitching_appearances = get_stdin("pitcher.pitching_appearances")?;
    // rule.pitcher.games_started = get_stdin("pitcher.games_started")?;
    rule.pitcher.innings_pitched = get_stdin("pitcher.innings_pitched")?;
    rule.pitcher.wins = get_stdin("pitcher.wins")?;
    // rule.pitcher.losses = get_stdin("pitcher.losses")?;
    // rule.pitcher.complete_games = get_stdin("pitcher.complete_games")?;
    // rule.pitcher.shutouts = get_stdin("pitcher.shutouts")?;
    rule.pitcher.saves = get_stdin("pitcher.saves")?;
    rule.pitcher.outs = get_stdin("pitcher.outs")?;
    rule.pitcher.hits = get_stdin("pitcher.hits")?;
    // rule.pitcher.total_batters_faced = get_stdin("pitcher.total_batters_faced")?;
    // rule.pitcher.runs = get_stdin("pitcher.runs")?;
    rule.pitcher.earned_runs = get_stdin("pitcher.earned_runs")?;
    // rule.pitcher.home_runs = get_stdin("pitcher.home_runs")?;
    // rule.pitcher.walks = get_stdin("pitcher.walks")?;
    // rule.pitcher.intentional_walks = get_stdin("pitcher.intentional_walks")?;
    rule.pitcher.hit_batters = get_stdin("pitcher.hit_batters")?;
    rule.pitcher.strikeouts = get_stdin("pitcher.strikeouts")?;
    // rule.pitcher.wild_pitches = get_stdin("pitcher.wild_pitches")?;
    // rule.pitcher.balks = get_stdin("pitcher.balks")?;
    // rule.pitcher.stolen_bases_allowed = get_stdin("pitcher.stolen_bases_allowed")?;
    // rule.pitcher.batters_grounded_into_double_plays =
    //     get_stdin("pitcher.batters_grounded_into_double_plays")?;
    // rule.pitcher.save_chances = get_stdin("pitcher.save_chances")?;
    // rule.pitcher.holds = get_stdin("pitcher.holds")?;
    // rule.pitcher.total_bases_allowed = get_stdin("pitcher.total_bases_allowed")?;
    // rule.pitcher.pitch_count = get_stdin("pitcher.pitch_count")?;
    // rule.pitcher.singles_allowed = get_stdin("pitcher.singles_allowed")?;
    // rule.pitcher.doubles_allowed = get_stdin("pitcher.doubles_allowed")?;
    // rule.pitcher.triples_allowed = get_stdin("pitcher.triples_allowed")?;
    // rule.pitcher.relief_wins = get_stdin("pitcher.relief_wins")?;
    // rule.pitcher.relief_losses = get_stdin("pitcher.relief_losses")?;
    // rule.pitcher.pickoffs = get_stdin("pitcher.pickoffs")?;
    // rule.pitcher.relief_appearances = get_stdin("pitcher.relief_appearances")?;
    // rule.pitcher.no_hitters = get_stdin("pitcher.no_hitters")?;
    // rule.pitcher.perfect_games = get_stdin("pitcher.perfect_games")?;
    // rule.pitcher.inherited_runners_scored = get_stdin("pitcher.inherited_runners_scored")?;
    // rule.pitcher.quality_starts = get_stdin("pitcher.quality_starts")?;
    // rule.pitcher.blown_saves = get_stdin("pitcher.blown_saves")?;
    // rule.pitcher.net_saves = get_stdin("pitcher.net_saves")?;
    // rule.pitcher.saves_and_holds = get_stdin("pitcher.saves_and_holds")?;
    // rule.pitcher.net_saves_and_holds = get_stdin("pitcher.net_saves_and_holds")?;

    Ok(rule)
}

fn get_stdin(label: &str) -> Result<f32, Box<dyn Error>> {
    let mut input_str = String::new();

    loop {
        input_str.clear();
        print!("Enter score for {} (enter for 0) > ", label);
        io::stdout().flush()?;
        if io::stdin().read_line(&mut input_str).is_err() {
            println!("Failed to read input, please retry.");
            continue;
        }

        let trimmed = input_str.trim();
        if trimmed.len() == 0 {
            return Ok(0.0);
        }
        match trimmed.parse::<f32>() {
            Ok(i) => {
                return Ok(i);
            }
            Err(_) => println!("Please input an integer. your input: {}", input_str),
        }
    }
}
