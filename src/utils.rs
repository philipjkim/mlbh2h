extern crate reqwest;
use std::error::Error;

pub fn get_json_res(url: &String) -> Result<String, Box<dyn Error>> {
    Ok(reqwest::get(url)?.text()?)
}
