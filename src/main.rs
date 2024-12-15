mod conf;
mod inject;
mod template;

use crate::{conf::load_conf, inject::*};

use std::time::Duration;

use argh::FromArgs;
use reqwest::blocking::Client;
use std::fs;
use std::io::Result;
use std::path::PathBuf;
use walkdir::WalkDir;

use colored::Colorize;

#[derive(FromArgs)]
/// Booru Tag Injector
struct Args {
    /// recursive traversal
    #[argh(switch, short = 'r')]
    recurse: bool,

    /// allow overwriting tags
    #[argh(switch, short = 'o')]
    overwrite: bool,

    /// file or folder path
    #[argh(positional)]
    fpath: PathBuf,
}

fn main() -> Result<()> {
    let booru_vec = match load_conf() {
        Ok(bv) => bv,
        Err(e) => {
            println!("{}", e.red());
            return Ok(());
        }
    };

    let args: Args = argh::from_env();

    let target_path = fs::canonicalize(args.fpath)?;
    if !target_path.exists() {
        println!("{}", "No such file or directory".red());
        return Ok(());
    }

    let client = Client::builder()
        .timeout(Duration::from_secs(10))
        .gzip(true)
        .build()
        .unwrap();

    let mut pf_vec = Vec::new();
    macro_rules! conveyor {
        ($epath:expr) => {
            if let Some(hash) = process($epath, args.overwrite) {
                if let Some(pl) = get_tags(&client, &booru_vec, hash.as_ref()) {
                    if let Some(put_failure) = insert($epath, pl) {
                        pf_vec.push(put_failure)
                    };
                }
            }
        };
    }

    if target_path.is_dir() {
        if args.recurse {
            for entry in WalkDir::new(target_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                conveyor!(entry.path());
            }
        } else {
            for entry in fs::read_dir(target_path)?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                conveyor!(&entry.path());
            }
        }
    } else {
        conveyor!(&target_path);
    }

    for pf in pf_vec {
        println!("{}", pf.red());
    }

    Ok(())
}
