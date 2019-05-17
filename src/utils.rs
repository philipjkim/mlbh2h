extern crate reqwest;
use std::env;
use std::error::Error;

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
}
