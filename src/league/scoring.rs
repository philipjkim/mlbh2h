use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BatterScoringRule {
    // pub games_played: f32,
    // pub games_started: f32,
    pub at_bats: f32,
    pub runs: f32,
    pub hits: f32,
    // pub singles: f32,
    // pub doubles: f32,
    // pub triples: f32,
    pub home_runs: f32,
    pub runs_batted_in: f32,
    // pub sacrifice_hits: f32,
    // pub sacrifice_flys: f32,
    pub stolen_bases: f32,
    // pub caught_stealing: f32,
    pub walks: f32,
    // pub intentilnal_walks: f32,
    pub hit_by_pitch: f32,
    // pub strikeouts: f32,
    // pub ground_into_double_play: f32,
    pub total_bases: f32,
    // pub putouts: f32,
    // pub assists: f32,
    // pub errors: f32,
    // pub extra_base_hits: f32,
    // pub net_stolen_bases: f32,
    // pub hitting_for_the_cycle: f32,
    // pub plate_appearances: f32,
    // pub grand_slam_home_runs: f32,
    // pub outfield_assists: f32,
    // pub double_plays_turned: f32,
    // pub catcher_interference: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PitcherScoringRule {
    // pub pitching_appearances: f32,
    // pub games_started: f32,
    pub innings_pitched: f32,
    pub wins: f32,
    // pub losses: f32,
    // pub complete_games: f32,
    // pub shutouts: f32,
    pub saves: f32,
    pub outs: f32,
    pub hits: f32,
    // pub total_batters_faced: f32,
    // pub runs: f32,
    pub earned_runs: f32,
    // pub home_runs: f32,
    pub walks: f32,
    // pub intentional_walks: f32,
    pub hit_batters: f32,
    pub strikeouts: f32,
    // pub wild_pitches: f32,
    // pub balks: f32,
    // pub stolen_bases_allowed: f32,
    // pub batters_grounded_into_double_plays: f32,
    // pub save_chances: f32,
    // pub holds: f32,
    // pub total_bases_allowed: f32,
    // pub pitch_count: f32,
    // pub singles_allowed: f32,
    // pub doubles_allowed: f32,
    // pub triples_allowed: f32,
    // pub relief_wins: f32,
    // pub relief_losses: f32,
    // pub pickoffs: f32,
    // pub relief_appearances: f32,
    // pub no_hitters: f32,
    // pub perfect_games: f32,
    // pub inherited_runners_scored: f32,
    // pub quality_starts: f32,
    // pub blown_saves: f32,
    // pub net_saves: f32,
    // pub saves_and_holds: f32,
    // pub net_saves_and_holds: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScoringRule {
    pub batter: BatterScoringRule,
    pub pitcher: PitcherScoringRule,
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
    rule.batter.walks = get_stdin("batter.walks")?;
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
    rule.pitcher.walks = get_stdin("pitcher.walks")?;
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

pub fn load(league_name: &String) -> Result<ScoringRule, Box<dyn Error>> {
    let filepath = format!("data/{}/scoring.json", league_name);
    println!("Loading the scoring rule from file {}", filepath);
    let json = fs::read_to_string(filepath)?;
    Ok(serde_json::from_str(&json)?)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn load_should_return_scoring_rule_for_given_league() {
        let scoring = load(&"sample".to_owned()).unwrap();
        println!("scoring: {:#?}", scoring);

        assert_eq!(0.5, scoring.batter.hits);
        assert_eq!(-0.5, scoring.pitcher.earned_runs);
    }
}
