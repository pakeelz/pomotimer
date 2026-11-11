use std::{env::home_dir, path::PathBuf};

use tray_icon::{TrayIconBuilder, menu::Menu};

pub enum IconState {
    Timer,
    Breaks,
}

pub fn load_icon(icon: IconState) -> anyhow::Result<tray_icon::Icon> {
    let home = home_dir().unwrap_or_else(|| PathBuf::from("."));
    let mut timer_icon = home;

    match icon {
        IconState::Timer => timer_icon.push(".local/share/pomotimer/icon/timer.png"),
        IconState::Breaks => timer_icon.push(".local/share/pomotimer/icon/breaks.png"),
    }

    let image = image::open(timer_icon)?.to_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Ok(tray_icon::Icon::from_rgba(rgba, width, height)?)
}

pub fn register_tray(icon: IconState) -> anyhow::Result<tray_icon::TrayIcon> {
    gtk::init()?;
    let tray_menu = Menu::new();

    let icon = load_icon(icon)?;
    anyhow::Ok(
        TrayIconBuilder::new()
            .with_menu(Box::new(tray_menu))
            .with_icon(icon)
            .with_tooltip("Pomotimer")
            .build()?,
    )
}
