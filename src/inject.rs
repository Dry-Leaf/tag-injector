use std::path::Path;
use std::sync::LazyLock;
use std::{fs, io};

use crate::{conf::Booru, template::build_xmp};

use colored::Colorize;
use md5::{Digest, Md5};
use regex::Regex;
use reqwest::blocking::Client;
use xmp_toolkit::{xmp_ns::DC, XmpMeta};

static EXTENSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\.[^\.]*+$"#).unwrap());
static MD5SUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[a-f0-9]{32}$"#).unwrap());

pub fn get_tags(client: &Client, boards: &Vec<Booru>, md5sum: &str) -> Option<String> {
    for booru in boards {
        let Booru {
            name,
            api_url,
            tag_regex,
        } = booru;

        let full_url = api_url.to_string() + md5sum;
        let body = match client.get(full_url).send() {
            Ok(resp) => resp.text().unwrap(),
            Err(e) => {
                println!(
                    "Error sending request to {}: {}",
                    name.red(),
                    e.to_string().red()
                );
                continue;
            }
        };

        let present = tag_regex.captures(body.as_ref());
        if let Some(groups) = present {
            println!("File found on: {}", name);
            return Some(build_xmp(&groups[1]));
        }
    }

    println!("File not found\n");
    None
}

pub fn process(fpath: &Path) -> Option<String> {
    let fname = fpath.file_name().unwrap().to_str().unwrap();

    let ext_opt = EXTENSION_REGEX.find(fname);
    let (ext, ext_srt) = match ext_opt {
        Some(ext) => (ext.as_str().to_lowercase(), ext.start()),
        None => return None,
    };

    if !matches!(ext.as_ref(), ".jpg" | ".jpeg" | ".png" | ".gif" | ".jxl") {
        println!("Bad ext Current file: {}", fname);
        return None;
    }

    println!("Current file: {}", fname);

    // checking if file already has tags
    let xmp_opt = XmpMeta::from_file(fpath);
    if let Ok(xmp_d) = xmp_opt {
        if xmp_d.contains_property(DC, "subject") {
            return None;
        }
    }

    // checking for an md5 hash in the file name
    let fname_stem = &fname[..ext_srt];
    let md5_opt = MD5SUM_REGEX.find(fname_stem);

    let md5sum = if let Some(val) = md5_opt {
        val.as_str()
    } else {
        let mut file = fs::File::open(&fpath).unwrap();
        let mut hasher = Md5::new();

        io::copy(&mut file, &mut hasher).unwrap();
        &format!("{:x}", hasher.finalize())
    };

    return Some(md5sum.to_string());
}
