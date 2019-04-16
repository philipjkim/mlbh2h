use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::io::{self, prelude::*};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct BatterScoringRule {
    pub at_bats: f32,
    pub runs: f32,
    pub hits: f32,
    pub singles: f32,
    pub doubles: f32,
    pub triples: f32,
    pub home_runs: f32,
    pub runs_batted_in: f32,
    pub sacrifice_hits: f32,
    pub stolen_bases: f32,
    pub caught_stealing: f32,
    pub walks: f32,
    pub intentional_walks: f32,
    pub hit_by_pitch: f32,
    pub strikeouts: f32,
    pub ground_into_double_play: f32,
    pub total_bases: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct PitcherScoringRule {
    pub innings_pitched: f32,
    pub wins: f32,
    pub losses: f32,
    pub complete_games: f32,
    pub shutouts: f32,
    pub saves: f32,
    pub outs: f32,
    pub hits: f32,
    pub earned_runs: f32,
    pub home_runs: f32,
    pub walks: f32,
    pub intentional_walks: f32,
    pub hit_batters: f32,
    pub strikeouts: f32,
    pub stolen_bases_allowed: f32,
    pub batters_grounded_into_double_plays: f32,
    pub total_bases_allowed: f32,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ScoringRule {
    pub batter: BatterScoringRule,
    pub pitcher: PitcherScoringRule,
}

impl ScoringRule {
    pub fn header_items(&self) -> Vec<String> {
        let mut items = vec![
            "Player".to_owned(),
            "Team".to_owned(),
            "FanPts".to_owned(),
            "Pos".to_owned(),
        ];

        if self.batter.at_bats != 0.0 {
            items.push("B.AB".to_owned());
        }
        if self.batter.runs != 0.0 {
            items.push("B.R".to_owned());
        }
        if self.batter.hits != 0.0 {
            items.push("B.H".to_owned());
        }
        if self.batter.singles != 0.0 {
            items.push("B.1B".to_owned());
        }
        if self.batter.doubles != 0.0 {
            items.push("B.2B".to_owned());
        }
        if self.batter.triples != 0.0 {
            items.push("B.3B".to_owned());
        }
        if self.batter.home_runs != 0.0 {
            items.push("B.HR".to_owned());
        }
        if self.batter.runs_batted_in != 0.0 {
            items.push("B.RBI".to_owned());
        }
        if self.batter.sacrifice_hits != 0.0 {
            items.push("B.SAC".to_owned());
        }
        if self.batter.stolen_bases != 0.0 {
            items.push("B.SB".to_owned());
        }
        if self.batter.caught_stealing != 0.0 {
            items.push("B.CS".to_owned());
        }
        if self.batter.walks != 0.0 {
            items.push("B.BB".to_owned());
        }
        if self.batter.intentional_walks != 0.0 {
            items.push("B.IBB".to_owned());
        }
        if self.batter.hit_by_pitch != 0.0 {
            items.push("B.HBP".to_owned());
        }
        if self.batter.strikeouts != 0.0 {
            items.push("B.K".to_owned());
        }
        if self.batter.ground_into_double_play != 0.0 {
            items.push("B.GIDP".to_owned());
        }
        if self.batter.total_bases != 0.0 {
            items.push("B.TB".to_owned());
        }

        if self.pitcher.innings_pitched != 0.0 {
            items.push("P.IP".to_owned());
        }
        if self.pitcher.wins != 0.0 {
            items.push("P.W".to_owned());
        }
        if self.pitcher.losses != 0.0 {
            items.push("P.L".to_owned());
        }
        if self.pitcher.complete_games != 0.0 {
            items.push("P.CG".to_owned());
        }
        if self.pitcher.shutouts != 0.0 {
            items.push("P.SHO".to_owned());
        }
        if self.pitcher.saves != 0.0 {
            items.push("P.SV".to_owned());
        }
        if self.pitcher.outs != 0.0 {
            items.push("P.OUT".to_owned());
        }
        if self.pitcher.hits != 0.0 {
            items.push("P.H".to_owned());
        }
        if self.pitcher.earned_runs != 0.0 {
            items.push("P.ER".to_owned());
        }
        if self.pitcher.home_runs != 0.0 {
            items.push("P.HR".to_owned());
        }
        if self.pitcher.walks != 0.0 {
            items.push("P.BB".to_owned());
        }
        if self.pitcher.intentional_walks != 0.0 {
            items.push("P.IBB".to_owned());
        }
        if self.pitcher.hit_batters != 0.0 {
            items.push("P.HBP".to_owned());
        }
        if self.pitcher.strikeouts != 0.0 {
            items.push("P.K".to_owned());
        }
        if self.pitcher.stolen_bases_allowed != 0.0 {
            items.push("P.SB".to_owned());
        }
        if self.pitcher.batters_grounded_into_double_plays != 0.0 {
            items.push("P.GIDP".to_owned());
        }
        if self.pitcher.total_bases_allowed != 0.0 {
            items.push("P.TB".to_owned());
        }

        items
    }
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

    rule.batter.at_bats = get_stdin("batter.at_bats")?;
    rule.batter.runs = get_stdin("batter.runs")?;
    rule.batter.hits = get_stdin("batter.hits")?;
    rule.batter.singles = get_stdin("batter.singles")?;
    rule.batter.doubles = get_stdin("batter.doubles")?;
    rule.batter.triples = get_stdin("batter.triples")?;
    rule.batter.home_runs = get_stdin("batter.home_runs")?;
    rule.batter.runs_batted_in = get_stdin("batter.runs_batted_in")?;
    rule.batter.sacrifice_hits = get_stdin("batter.sacrifice_hits")?;
    rule.batter.stolen_bases = get_stdin("batter.stolen_bases")?;
    rule.batter.caught_stealing = get_stdin("batter.caught_stealing")?;
    rule.batter.walks = get_stdin("batter.walks")?;
    rule.batter.intentional_walks = get_stdin("batter.intentional_walks")?;
    rule.batter.hit_by_pitch = get_stdin("batter.hit_by_pitch")?;
    rule.batter.strikeouts = get_stdin("batter.strikeouts")?;
    rule.batter.ground_into_double_play = get_stdin("batter.ground_into_double_play")?;
    rule.batter.total_bases = get_stdin("batter.total_bases")?;

    rule.pitcher.innings_pitched = get_stdin("pitcher.innings_pitched")?;
    rule.pitcher.wins = get_stdin("pitcher.wins")?;
    rule.pitcher.losses = get_stdin("pitcher.losses")?;
    rule.pitcher.complete_games = get_stdin("pitcher.complete_games")?;
    rule.pitcher.shutouts = get_stdin("pitcher.shutouts")?;
    rule.pitcher.saves = get_stdin("pitcher.saves")?;
    rule.pitcher.outs = get_stdin("pitcher.outs")?;
    rule.pitcher.hits = get_stdin("pitcher.hits")?;
    rule.pitcher.earned_runs = get_stdin("pitcher.earned_runs")?;
    rule.pitcher.home_runs = get_stdin("pitcher.home_runs")?;
    rule.pitcher.walks = get_stdin("pitcher.walks")?;
    rule.pitcher.intentional_walks = get_stdin("pitcher.intentional_walks")?;
    rule.pitcher.hit_batters = get_stdin("pitcher.hit_batters")?;
    rule.pitcher.strikeouts = get_stdin("pitcher.strikeouts")?;
    rule.pitcher.stolen_bases_allowed = get_stdin("pitcher.stolen_bases_allowed")?;
    rule.pitcher.batters_grounded_into_double_plays =
        get_stdin("pitcher.batters_grounded_into_double_plays")?;
    rule.pitcher.total_bases_allowed = get_stdin("pitcher.total_bases_allowed")?;

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
    if league_name == "sample" {
        return Ok(sample_scoring_rule());
    }

    let filepath = format!("data/{}/scoring.json", league_name);
    println!("Loading the scoring rule from file {}", filepath);
    let json = fs::read_to_string(filepath)?;
    Ok(serde_json::from_str(&json)?)
}

pub fn sample_scoring_rule() -> ScoringRule {
    ScoringRule {
        batter: BatterScoringRule {
            at_bats: 0.0,
            runs: 2.0,
            hits: 0.5,
            singles: 0.0,
            doubles: 0.0,
            triples: 0.0,
            home_runs: 4.0,
            runs_batted_in: 2.0,
            sacrifice_hits: 0.0,
            stolen_bases: 2.0,
            caught_stealing: 0.0,
            walks: 0.0,
            intentional_walks: 0.0,
            hit_by_pitch: 0.0,
            strikeouts: 0.0,
            ground_into_double_play: 0.0,
            total_bases: 0.0,
        },
        pitcher: PitcherScoringRule {
            innings_pitched: 1.0,
            wins: 5.0,
            losses: 0.0,
            complete_games: 0.0,
            shutouts: 0.0,
            saves: 5.0,
            outs: 0.0,
            hits: 0.0,
            earned_runs: -0.5,
            home_runs: 0.0,
            walks: 0.0,
            intentional_walks: 0.0,
            hit_batters: 0.0,
            strikeouts: 2.0,
            stolen_bases_allowed: 0.0,
            batters_grounded_into_double_plays: 0.0,
            total_bases_allowed: 0.0,
        },
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    fn load_should_return_sample_scoring_rule_when_league_name_is_sample() {
        let scoring = load(&"sample".to_owned()).unwrap();

        assert_eq!(0.5, scoring.batter.hits);
        assert_eq!(-0.5, scoring.pitcher.earned_runs);
    }

    #[test]
    fn header_items_should_return_header_items_to_display() {
        let scoring = sample_scoring_rule();
        let items = scoring.header_items();

        assert_eq!(14, items.len());
        assert_eq!(
            "Player,Team,FanPts,Pos,B.R,B.H,B.HR,B.RBI,B.SB,P.IP,P.W,P.SV,P.ER,P.K".to_owned(),
            items.join(",")
        );
    }
}
