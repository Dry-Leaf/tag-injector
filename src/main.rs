mod conf;
mod inject;

use crate::inject::process;

use argh::FromArgs;
use std::fs;
use std::io::Result;
use std::path::Path;
use walkdir::WalkDir;

use colored::Colorize;

#[derive(FromArgs)]
/// Booru Tag Injector
struct Args {
    /// recursive traversal
    #[argh(switch, short = 'r')]
    recurse: bool,

    /// file or folder path
    #[argh(positional)]
    fpath: String,
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();

    let target_path = Path::new(&args.fpath);
    if !target_path.exists() {
        println!("{}", "No such file or directory".red());
        return Ok(());
    }

    if target_path.is_dir() {
        if args.recurse {
            for entry in WalkDir::new(target_path)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file())
            {
                process(entry.path())
            }
        } else {
            for entry in fs::read_dir(target_path)?
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_file())
            {
                process(&entry.path())
            }
        }
    } else {
        process(target_path)
    }
    Ok(())
}
