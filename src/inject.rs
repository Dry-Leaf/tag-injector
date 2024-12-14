use std::path::Path;
use std::str::FromStr;
use std::sync::LazyLock;
use std::thread::sleep;
use std::time::Duration;
use std::{fs, io};

use crate::{conf::Booru, template::build_xmp};

use colored::Colorize;
use crossterm::cursor;
use md5::{Digest, Md5};
use rand::{thread_rng, Rng};
use regex::Regex;
use reqwest::{blocking::Client, header};
use spinners::{Spinner, Spinners};
use xmp_toolkit::{xmp_ns::DC, OpenFileOptions, XmpFile, XmpMeta};

static EXTENSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\.[^\.]*+$"#).unwrap());
static MD5SUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[a-f0-9]{32}$"#).unwrap());

pub fn insert(fpath: &Path, pl: String) -> Option<String> {
    let mut xmp_con = XmpFile::new().unwrap();
    let xmp_opt = xmp_con.open_file(
        fpath,
        OpenFileOptions::default()
            .for_update()
            .optimize_file_layout(),
    );
    xmp_opt.expect("Xmp Toolkit failed to open file");

    let put_failure = if let Ok(new_xmp) = XmpMeta::from_str(pl.as_ref()) {
        if let Err(ierr) = xmp_con.put_xmp(&new_xmp) {
            Some(format!(
                "Failed to insert xmp data on {}: {}",
                fpath.to_str().unwrap(),
                ierr
            ))
        } else {
            None
        }
    } else {
        Some("Failied to form XmpMeta".into())
    };

    xmp_con.close();
    if put_failure.is_none() {
        println!("Tags added\n");
    } else {
        println!("Error occured\n");
    }
    put_failure
}

pub fn get_tags(client: &Client, boards: &Vec<Booru>, md5sum: &str) -> Option<String> {
    let rwait: u64 = thread_rng().gen_range(5..=10);

    crossterm::execute!(std::io::stdout(), cursor::Hide).unwrap();
    let mut sp = Spinner::new(Spinners::Point, "Hold Tight".into());
    sleep(Duration::from_secs(rwait));

    sp.stop_with_newline();
    crossterm::execute!(std::io::stdout(), cursor::Show).unwrap();

    for booru in boards {
        let Booru {
            name,
            api_url,
            tag_regex,
        } = booru;

        let full_url = api_url.to_string() + md5sum;
        let body = match client
            .get(full_url)
            .header(header::USER_AGENT, "curl/7.87.0")
            .header(header::ACCEPT, "text/*")
            .send()
        {
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

pub fn process(fpath: &Path, overwrite: bool) -> Option<String> {
    let fname = fpath.file_name().unwrap().to_str().unwrap();

    let ext_opt = EXTENSION_REGEX.find(fname);
    let (ext, ext_srt) = match ext_opt {
        Some(ext) => (ext.as_str().to_lowercase(), ext.start()),
        None => return None,
    };

    if !matches!(ext.as_ref(), ".jpg" | ".jpeg" | ".png" | ".gif" | ".jxl") {
        return None;
    }

    println!("Current file: {}", fname);

    // checking if file already has tags
    if !overwrite {
        let xmp_opt = XmpMeta::from_file(fpath);
        if let Ok(xmp_d) = xmp_opt {
            if xmp_d.contains_property(DC, "subject") {
                println!("Already tagged\n");
                return None;
            }
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
