use std::process::Command;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::collect_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Filter {
    pub mode: String,
    pub name: String,
    pub io_type: String,
    pub help: String,
}

pub fn ffmpeg_filters(ffmpeg_bin: &str) -> Result<Vec<Filter>> {
    let output = Command::new(ffmpeg_bin)
        .arg("-loglevel")
        .arg("quiet")
        .arg(format!("-filters"))
        .output()?;

    let help = String::from_utf8_lossy(&output.stdout);

    let filters = help
        .lines()
        .filter(|line| line.starts_with(" "))
        .filter(|line| !line.starts_with("  "))
        .map(str::trim)
        .map(str::split_whitespace)
        .filter_map(|mut split| {
            let mode = split.next()?.to_string();
            let name = split.next()?.to_string();
            let io_type = split.next()?.to_string();
            let help = split.fold(String::new(), collect_string);

            Some(Filter {
                name,
                help,
                mode,
                io_type,
            })
        })
        .collect();

    Ok(filters)
}
