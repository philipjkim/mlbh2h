use crate::stats::FantasyPlayer;
use std::collections::HashMap;

pub fn get_header_string(headers: &[String], is_csv: bool) -> String {
    if is_csv {
        return headers.join(",");
    }

    // for pretty-print format
    let mut items = Vec::new();
    for i in headers {
        if i.contains("Player") {
            items.push(format!("{:18}", "Player")); // 18bytes
        } else if i.contains("Team") {
            items.push(format!("{:10}", "Team")); // 10bytes
        } else if i.contains("P.IP") {
            items.push(format!("{}   ", i));
        } else {
            items.push(format!("{} ", i));
        }
    }

    items.join("")
}

#[allow(clippy::cognitive_complexity)]
pub fn get_player_stats_string(fp: &FantasyPlayer, headers: &[String], is_csv: bool) -> String {
    let bstats = &fp.player.batter_stats;
    let pstats = &fp.player.pitcher_stats;

    if is_csv {
        headers
            .iter()
            .map(|h| match h.as_str() {
                "Player" => fp.player.name.to_owned().into(),
                "Team" => (*fp.team).to_owned().into(),
                "FanPts" => format!("{:.2}", fp.fantasy_points),
                "Pos" => fp.player.primary_position.to_owned().into(),
                "B.AB" => {
                    if let Some(s) = bstats {
                        s.at_bats.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.R" => {
                    if let Some(s) = bstats {
                        s.runs.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.H" => {
                    if let Some(s) = bstats {
                        s.hits.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.1B" => {
                    if let Some(s) = bstats {
                        s.singles.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.2B" => {
                    if let Some(s) = bstats {
                        s.doubles.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.3B" => {
                    if let Some(s) = bstats {
                        s.triples.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.HR" => {
                    if let Some(s) = bstats {
                        s.home_runs.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.RBI" => {
                    if let Some(s) = bstats {
                        s.runs_batted_in.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.SAC" => {
                    if let Some(s) = bstats {
                        s.sacrifice_hits.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.SB" => {
                    if let Some(s) = bstats {
                        s.stolen_bases.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.CS" => {
                    if let Some(s) = bstats {
                        s.caught_stealing.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.BB" => {
                    if let Some(s) = bstats {
                        s.walks.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.IBB" => {
                    if let Some(s) = bstats {
                        s.intentional_walks.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.HBP" => {
                    if let Some(s) = bstats {
                        s.hit_by_pitch.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.K" => {
                    if let Some(s) = bstats {
                        s.strikeouts.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.GIDP" => {
                    if let Some(s) = bstats {
                        s.ground_into_double_play.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "B.TB" => {
                    if let Some(s) = bstats {
                        s.total_bases.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.IP" => {
                    if let Some(s) = pstats {
                        s.innings_pitched.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.W" => {
                    if let Some(s) = pstats {
                        s.wins.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.L" => {
                    if let Some(s) = pstats {
                        s.losses.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.CG" => {
                    if let Some(s) = pstats {
                        s.complete_games.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.SHO" => {
                    if let Some(s) = pstats {
                        s.shutouts.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.SV" => {
                    if let Some(s) = pstats {
                        s.saves.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.OUT" => {
                    if let Some(s) = pstats {
                        s.outs.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.H" => {
                    if let Some(s) = pstats {
                        s.hits.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.ER" => {
                    if let Some(s) = pstats {
                        s.earned_runs.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.HR" => {
                    if let Some(s) = pstats {
                        s.home_runs.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.BB" => {
                    if let Some(s) = pstats {
                        s.walks.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.IBB" => {
                    if let Some(s) = pstats {
                        s.intentional_walks.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.HBP" => {
                    if let Some(s) = pstats {
                        s.hit_batters.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.K" => {
                    if let Some(s) = pstats {
                        s.strikeouts.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.SB" => {
                    if let Some(s) = pstats {
                        s.stolen_bases_allowed.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.GIDP" => {
                    if let Some(s) = pstats {
                        s.batters_grounded_into_double_plays.to_string()
                    } else {
                        "".to_string()
                    }
                }
                "P.TB" => {
                    if let Some(s) = pstats {
                        s.total_bases_allowed.to_string()
                    } else {
                        "".to_string()
                    }
                }
                _ => "".to_string(),
            })
            .collect::<Vec<_>>()
            .join(",")
    } else {
        headers
            .iter()
            .map(|h| match h.as_str() {
                "Player" => {
                    let mut name = fp.player.name.to_owned().to_string();
                    let mut new_len = 17;
                    while !name.is_char_boundary(new_len) {
                        new_len -= 1;
                    }
                    name.truncate(new_len);
                    format!("{:18}", name)
                }
                "Team" => {
                    let mut team = (*fp.team).to_owned().to_string();
                    team.truncate(9);
                    format!("{:10}", team)
                }
                "FanPts" => format!("{:6.2} ", fp.fantasy_points),
                "Pos" => format!("{:4}", fp.player.primary_position.to_owned().to_string()),
                "B.AB" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.at_bats.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.R" => {
                    if let Some(s) = bstats {
                        format!("{:4}", s.runs.to_string())
                    } else {
                        format!("{:4}", "")
                    }
                }
                "B.H" => {
                    if let Some(s) = bstats {
                        format!("{:4}", s.hits.to_string())
                    } else {
                        format!("{:4}", "")
                    }
                }
                "B.1B" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.singles.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.2B" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.doubles.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.3B" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.triples.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.HR" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.home_runs.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.RBI" => {
                    if let Some(s) = bstats {
                        format!("{:6}", s.runs_batted_in.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "B.SAC" => {
                    if let Some(s) = bstats {
                        format!("{:6}", s.sacrifice_hits.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "B.SB" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.stolen_bases.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.CS" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.caught_stealing.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.BB" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.walks.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "B.IBB" => {
                    if let Some(s) = bstats {
                        format!("{:6}", s.intentional_walks.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "B.HBP" => {
                    if let Some(s) = bstats {
                        format!("{:6}", s.hit_by_pitch.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "B.K" => {
                    if let Some(s) = bstats {
                        format!("{:4}", s.strikeouts.to_string())
                    } else {
                        format!("{:4}", "")
                    }
                }
                "B.GIDP" => {
                    if let Some(s) = bstats {
                        format!("{:7}", s.ground_into_double_play.to_string())
                    } else {
                        format!("{:7}", "")
                    }
                }
                "B.TB" => {
                    if let Some(s) = bstats {
                        format!("{:5}", s.total_bases.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "P.IP" => {
                    if let Some(s) = pstats {
                        let ip = (s.innings_pitched * 10.0).round() / 10.0;
                        format!("{:7}", ip.to_string())
                    } else {
                        format!("{:7}", "")
                    }
                }
                "P.W" => {
                    if let Some(s) = pstats {
                        format!("{:4}", s.wins.to_string())
                    } else {
                        format!("{:4}", "")
                    }
                }
                "P.L" => {
                    if let Some(s) = pstats {
                        format!("{:4}", s.losses.to_string())
                    } else {
                        format!("{:4}", "")
                    }
                }
                "P.CG" => {
                    if let Some(s) = pstats {
                        format!("{:5}", s.complete_games.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "P.SHO" => {
                    if let Some(s) = pstats {
                        format!("{:6}", s.shutouts.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "P.SV" => {
                    if let Some(s) = pstats {
                        format!("{:5}", s.saves.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "P.OUT" => {
                    if let Some(s) = pstats {
                        format!("{:6}", s.outs.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "P.H" => {
                    if let Some(s) = pstats {
                        format!("{:4}", s.hits.to_string())
                    } else {
                        format!("{:4}", "")
                    }
                }
                "P.ER" => {
                    if let Some(s) = pstats {
                        format!("{:5}", s.earned_runs.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "P.HR" => {
                    if let Some(s) = pstats {
                        format!("{:5}", s.home_runs.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "P.BB" => {
                    if let Some(s) = pstats {
                        format!("{:5}", s.walks.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "P.IBB" => {
                    if let Some(s) = pstats {
                        format!("{:6}", s.intentional_walks.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "P.HBP" => {
                    if let Some(s) = pstats {
                        format!("{:6}", s.hit_batters.to_string())
                    } else {
                        format!("{:6}", "")
                    }
                }
                "P.K" => {
                    if let Some(s) = pstats {
                        format!("{:4}", s.strikeouts.to_string())
                    } else {
                        format!("{:4}", "")
                    }
                }
                "P.SB" => {
                    if let Some(s) = pstats {
                        format!("{:5}", s.stolen_bases_allowed.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                "P.GIDP" => {
                    if let Some(s) = pstats {
                        format!("{:7}", s.batters_grounded_into_double_plays.to_string())
                    } else {
                        format!("{:7}", "")
                    }
                }
                "P.TB" => {
                    if let Some(s) = pstats {
                        format!("{:5}", s.total_bases_allowed.to_string())
                    } else {
                        format!("{:5}", "")
                    }
                }
                _ => "".to_string(),
            })
            .collect::<Vec<_>>()
            .join("")
    }
}

pub fn print_scores_per_team(players: Vec<FantasyPlayer>, is_csv: bool) {
    let mut scores = players
        .iter()
        .fold(HashMap::new(), |mut acc, x| {
            let fp = acc.entry(x.team.clone()).or_insert(0.0);
            *fp += x.fantasy_points;
            acc
        })
        .into_iter()
        .map(|(k, v)| (k, v))
        .collect::<Vec<_>>();
    scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    println!("# Team Rankings");
    if is_csv {
        println!("Team,FanPts");
        scores.into_iter().for_each(|(team, pts)| {
            println!("{},{}", team, pts);
        });
    } else {
        println!("{:20}{:>8}", "Team", "FanPts");
        scores.into_iter().for_each(|(team, pts)| {
            println!("{:20}{:8.1}", team, pts);
        });
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::stats::{self, BatterStats, FantasyPlayer, PitcherStats, Player};

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
    fn get_header_string_should_return_csv_or_pretty_format_string() {
        let headers: Vec<_> = vec!["Player", "Team", "FanPts", "Pos", "B.HR", "P.K"]
            .into_iter()
            .map(std::string::ToString::to_string)
            .collect();

        let csv = get_header_string(&headers, true);
        assert_eq!("Player,Team,FanPts,Pos,B.HR,P.K", csv);

        let pretty = get_header_string(&headers, false);
        assert_eq!("Player            Team      FanPts Pos B.HR P.K ", pretty);
    }

    #[test]
    fn fantasy_player_get_stats_string_should_return_string() {
        use crate::league::scoring::sample_scoring_rule;
        use std::borrow::Cow;
        use std::rc::Rc;

        let sr = sample_scoring_rule();

        let header_items = sr.get_header_items();

        let batter = mock_batter();
        let fp = FantasyPlayer {
            team: Rc::new(Cow::Borrowed("Avengers")),
            fantasy_points: stats::get_fantasy_points(&batter, &sr),
            player: batter,
        };

        assert_eq!(
            "Trey Mancini,Avengers,13.50,RF,2,3,1,2,0,,,,,",
            get_player_stats_string(&fp, &header_items, true)
        );
        assert_eq!(
            "Trey Mancini      Avengers   13.50 RF  2   3   1    2     0                             ",
            get_player_stats_string(&fp, &header_items, false)
        );

        let pitcher = mock_pitcher();
        let fp = FantasyPlayer {
            team: Rc::new(Cow::Borrowed("Avengers")),
            fantasy_points: stats::get_fantasy_points(&pitcher, &sr),
            player: pitcher,
        };

        assert_eq!(
            "Blake Snell,Avengers,32.50,SP,,,,,,6,1,0,1,11",
            get_player_stats_string(&fp, &header_items, true)
        );
        assert_eq!(
            "Blake Snell       Avengers   32.50 SP                          6      1   0    1    11  ",
            get_player_stats_string(&fp, &header_items, false)
        );
    }
}
