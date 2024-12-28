use std::process::Command;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PixFmt {
    pub flags: String,
    pub name: String,
    pub nb_components: usize,
    pub bits_per_pixel: usize,
    pub bit_depths: String,
}

pub fn ffmpeg_pix_fmts(ffmpeg_bin: &str) -> Result<Vec<PixFmt>> {
    let output = Command::new(ffmpeg_bin)
        .arg("-loglevel")
        .arg("quiet")
        .arg(format!("-pix_fmts"))
        .output()?;

    let help = String::from_utf8_lossy(&output.stdout);

    let help = help
        .split_once("-----")
        .ok_or_else(|| anyhow!("failed to parse pix_fmt"))?
        .1;

    let filters = help
        .lines()
        .map(str::trim)
        .map(str::split_whitespace)
        .filter_map(|mut split| {
            let flags = split.next()?.to_string();
            let name = split.next()?.to_string();
            let nb_components = split.next()?.parse().ok()?;
            let bits_per_pixel = split.next()?.parse().ok()?;
            let bit_depths = split.next()?.to_string();

            Some(PixFmt {
                flags,
                name,
                nb_components,
                bits_per_pixel,
                bit_depths,
            })
        })
        .collect();

    Ok(filters)
}
