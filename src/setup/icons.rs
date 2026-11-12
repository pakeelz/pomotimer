use std::path::PathBuf;

use crate::setup::download_assets;

pub async fn handle_icon_dir(home: PathBuf) -> anyhow::Result<()> {
    let mut icon_dir = home;
    icon_dir.push(".local/share/pomotimer/icon");

    // Check timer.png
    icon_dir.push("timer.png");

    let url = "https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/timer.png";
    handle_icon(&icon_dir, url).await?;

    // Check breaks.png
    icon_dir.pop();
    icon_dir.push("breaks.png");

    let url = "https://raw.githubusercontent.com/pakeelz/pomotimer/main/assets/breaks.png";
    handle_icon(&icon_dir, url).await?;
    Ok(())
}

async fn handle_icon(icon_dir: &PathBuf, url: &str) -> anyhow::Result<()> {
    if !icon_dir.exists() {
        if let Some(basename) = icon_dir.iter().last() {
            println!("Downloading assets: {}", basename.to_string_lossy());
        }
        download_assets(url, icon_dir).await?;
    } else {
        let buf = tokio::fs::read(&icon_dir).await.unwrap();
        if !infer::image::is_png(&buf) {
            if let Some(basename) = icon_dir.iter().last() {
                println!("Repairing assets: {}", basename.to_string_lossy());
            }
            download_assets(url, icon_dir).await?
        }
    }
    Ok(())
}
