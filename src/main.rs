use std::{fs, thread};

use anyhow::Result;
use strum::IntoEnumIterator;

use ffmpeg_gen::{
    filter::ffmpeg_filters,
    items::{ItemType, ffmpeg_items},
    pix_fmt::ffmpeg_pix_fmts,
};

const FFMPEG_BIN: Option<&str> = option_env!("FFMPEG_BIN");

fn main() -> Result<()> {
    let ffmpeg_bin = FFMPEG_BIN.unwrap_or("ffmpeg");

    let possible_filters = ffmpeg_filters(ffmpeg_bin)?;
    serde_json::to_writer_pretty(
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(format!("filters.json"))?,
        &possible_filters,
    )?;
    let pix_fmts = ffmpeg_pix_fmts(ffmpeg_bin)?;
    serde_json::to_writer_pretty(
        fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(false)
            .open(format!("pix_fmts.json"))?,
        &pix_fmts,
    )?;

    let mut tasks = Vec::new();
    for item in ItemType::iter() {
        tasks.push(thread::spawn(move || -> Result<()> {
            let item_name = item.to_string();
            let item_options = ffmpeg_items(ffmpeg_bin, item)?;

            serde_json::to_writer_pretty(
                fs::OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(false)
                    .open(format!("{item_name}s.json"))?,
                &item_options,
            )?;

            Ok(())
        }));
    }

    for task in tasks {
        let _ = task.join();
    }

    Ok(())
}
