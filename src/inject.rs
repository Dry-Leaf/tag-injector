use regex::Regex;
use std::path::Path;
use std::sync::LazyLock;
use std::{fs, io};

use md5::{Digest, Md5};
use xmp_toolkit::{xmp_ns::DC, XmpMeta};

static EXTENSION_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"\.[^\.]*+$"#).unwrap());
static MD5SUM_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r#"[a-f0-9]{32}$"#).unwrap());

fn get_tags(_md5sum: &str) -> Vec<&str> {
    vec!["test"]
}

pub fn process(fpath: &Path) {
    let fname = fpath.file_name().unwrap().to_str().unwrap();

    let ext_opt = EXTENSION_REGEX.find(fname);
    let (ext, ext_srt) = match ext_opt {
        Some(ext) => (ext.as_str().to_lowercase(), ext.start()),
        None => return,
    };

    if !matches!(ext.as_ref(), ".jpg" | ".jpeg" | ".png" | ".gif" | ".jxl") {
        return;
    }

    // checking if file already has tags
    let xmp_opt = XmpMeta::from_file(fpath);
    if let Ok(xmp_d) = xmp_opt {
        if xmp_d.contains_property(DC, "subject") {
            return;
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

    println!("{} - {}", fname_stem, md5sum);

    get_tags(md5sum);
}
