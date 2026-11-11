use std::path::PathBuf;

use crate::setup::download_assets;

pub async fn handle_sound_dir(home: PathBuf) {
    let mut sound_dir = home;
    sound_dir.push(".local/share/pomotimer/sounds");

    // Check bell.mp3
    sound_dir.push("bell.mp3");

    let url = "https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/bell.mp3";
    handle_sound(&sound_dir, url).await;

    // Check clink.mp3
    sound_dir.pop();
    sound_dir.push("clink.mp3");

    let url = "https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/clink.mp3";
    handle_sound(&sound_dir, url).await;
}

async fn handle_sound(sound_dir: &PathBuf, url: &str) {
    if !sound_dir.exists() {
        if let Some(basename) = sound_dir.iter().last() {
            println!("Downloading assets: {}", basename.to_string_lossy());
        }
        if let Err(_e) = download_assets(url, sound_dir).await {
            // Debug soon
        }
    } else {
        let buf = tokio::fs::read(&sound_dir).await.unwrap();
        if !infer::audio::is_mp3(&buf) {
            if let Some(basename) = sound_dir.iter().last() {
                println!("Repairing assets: {}", basename.to_string_lossy());
            }
            if let Err(_e) = download_assets(url, sound_dir).await {
                // Debug soon
            }
        }
    }
}
