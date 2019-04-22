extern crate reqwest;
use std::env;
use std::error::Error;

pub fn get_json_res(url: &String) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url)?.text()?)
}

pub fn get_home_dir() -> String {
    let home_dir = env::var("HOME").unwrap_or(".".to_string());
    if home_dir.ends_with("/") {
        return home_dir
            .chars()
            .into_iter()
            .take(home_dir.len() - 1)
            .collect();
    }
    home_dir
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn get_home_dir_should_return_string_not_ending_with_a_slash() {
        let home_dir = get_home_dir();
        assert_eq!(false, home_dir.ends_with("/"));
    }
}
