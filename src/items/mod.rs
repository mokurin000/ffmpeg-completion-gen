use std::process::Command;

use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter};

use crate::collect_string;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub name: String,
    pub mode: String,
    pub help: String,
}

#[derive(EnumIter, Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum ItemType {
    Encoder,
    Decoder,
    Muxer,
    Demuxer,
    Codec,
    Format,
    Device,
}

fn item_delimiter(item: ItemType) -> &'static str {
    match item {
        ItemType::Decoder | ItemType::Encoder | ItemType::Codec => "------",
        ItemType::Muxer | ItemType::Demuxer | ItemType::Format | ItemType::Device => "---",
    }
}

pub fn ffmpeg_items(ffmpeg_bin: &str, item: ItemType) -> Result<Vec<Item>> {
    let output = Command::new(ffmpeg_bin)
        .arg("-loglevel")
        .arg("quiet")
        .arg(format!("-{item}s"))
        .output()?;

    let help = String::from_utf8_lossy(&output.stdout);

    let help = help
        .split_once(item_delimiter(item))
        .ok_or_else(|| anyhow!("failed to parse item {item}"))?
        .1;

    let filters = help
        .lines()
        .map(str::trim)
        .map(str::split_whitespace)
        .filter_map(|mut split| {
            let mode = split.next()?.to_string();
            let name = split.next()?.to_string();
            let help = split.fold(String::new(), collect_string);
            Some(Item { mode, name, help })
        })
        .collect();

    Ok(filters)
}
