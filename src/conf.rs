use std::fs;

use regex::Regex;
use serde::Deserialize;
use toml;

#[derive(Debug, Deserialize)]
pub struct Booru {
    pub name: String,
    pub api_url: String,
    #[serde(with = "serde_regex")]
    pub tag_regex: Regex,
}

#[derive(Deserialize, Debug)]
struct BooruVec {
    boards: Vec<Booru>,
}

pub fn load_conf() -> Result<Vec<Booru>, String> {
    let conf_str = match fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(_) => {
            return Err("Could not read config.toml: ".into());
        }
    };

    let conf: BooruVec = match toml::from_str(&conf_str) {
        Ok(d) => d,
        Err(e) => {
            return Err("Could not deserialize config.toml: ".to_string() + e.message());
        }
    };

    Ok(conf.boards)
}
