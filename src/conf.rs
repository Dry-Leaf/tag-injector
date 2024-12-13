use std::fs;

use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Booru {
    name: String,
    api_url: String,
    #[serde(with = "serde_regex")]
    tag_regex: Regex,
}

static CONFIG: Lazy<Result<Vec<Booru>, String>> = Lazy::new(|| {
    let conf_str = match fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(_) => {
            return Err(String::from("Could not read config.toml"));
        }
    };

    let conf: Vec<Booru> = match toml::from_str(&conf_str) {
        Ok(d) => d,
        Err(_) => {
            return Err(String::from("Could not deserialize config.toml"));
        }
    };

    Ok(conf)
});
