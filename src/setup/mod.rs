#![allow(dead_code)]

use crate::setup::{icons::handle_icon_dir, sounds::handle_sound_dir};
use std::{env::home_dir, path::PathBuf};
use tokio::{fs::File, io::AsyncWriteExt};

mod icons;
mod sounds;

pub async fn setup() {
    let home_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
    // Debug Soon
    if let Err(_e) = handle_sound_dir(home_dir.clone()).await {}
    if let Err(_e) = handle_icon_dir(home_dir.clone()).await {}
}

async fn download_assets(url: &str, output_path: &PathBuf) -> anyhow::Result<()> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let mut file = File::create(output_path).await?;
    file.write_all(&bytes).await?;
    anyhow::Ok(())
}
