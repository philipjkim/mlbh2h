use chrono::prelude::*;
use std::env;
use std::error::Error;
use time::Duration;

pub fn get_json_res(url: &str) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url)?.text()?)
}

pub fn get_home_dir() -> String {
    let home_dir = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    if home_dir.ends_with('/') {
        return home_dir.chars().take(home_dir.len() - 1).collect();
    }
    home_dir
}

pub fn date_strs(date: &str, range: &str) -> Vec<String> {
    let last_dt = format!("{}T00:00:00Z", date)
        .parse::<DateTime<Utc>>()
        .expect("error parsing date string");
    let mut result = vec![last_dt.format("%Y-%m-%d").to_string()];

    let asg_dates = vec!["2019-07-08", "2019-07-09", "2019-07-10"];

    if range == "1d" {
        return result;
    }

    if range == "all" {
        let mut dt = Utc.ymd(2019, 3, 28).and_hms(0, 0, 0);
        while dt < last_dt {
            let date_str = dt.format("%Y-%m-%d").to_string();
            if !asg_dates.iter().any(|&d| d == &date_str[..]) {
                result.push(date_str);
            }
            dt = dt + Duration::days(1);
        }
        return result;
    }

    let days = match range {
        "1w" => 7,
        "2w" => 14,
        "1m" => 30,
        _ => 1,
    };
    let mut dt = last_dt - Duration::days(1);
    while result.len() < days {
        let date_str = dt.format("%Y-%m-%d").to_string();
        if !asg_dates.iter().any(|&d| d == &date_str[..]) {
            result.push(date_str);
        }
        dt = dt - Duration::days(1);
    }

    result
}

pub fn yesterday_str() -> &'static str {
    Box::leak(
        (Local::now() - Duration::days(1))
            .format("%Y-%m-%d")
            .to_string()
            .into_boxed_str(),
    )
}

#[allow(dead_code)]
pub fn assert_eq_f32(a: f32, b: f32) {
    let a = (a * 100.0) as i32;
    let b = (b * 100.0) as i32;
    assert_eq!(a, b);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_home_dir_should_return_string_not_ending_with_a_slash() {
        let home_dir = get_home_dir();
        assert_eq!(false, home_dir.ends_with('/'));
    }

    #[test]
    fn date_strs_should_return_vector_of_strings() {
        let result_1d = date_strs("2019-06-17", "1d");
        assert_eq!(vec!["2019-06-17".to_string()], result_1d);

        let result_all = date_strs("2019-04-02", "all");
        assert_eq!(
            vec![
                "2019-04-02".to_string(),
                "2019-03-28".to_string(),
                "2019-03-29".to_string(),
                "2019-03-30".to_string(),
                "2019-03-31".to_string(),
                "2019-04-01".to_string(),
            ],
            result_all
        );

        let result_1w = date_strs("2019-06-05", "1w");
        assert_eq!(
            vec![
                "2019-06-05".to_string(),
                "2019-06-04".to_string(),
                "2019-06-03".to_string(),
                "2019-06-02".to_string(),
                "2019-06-01".to_string(),
                "2019-05-31".to_string(),
                "2019-05-30".to_string(),
            ],
            result_1w
        );
    }
}
