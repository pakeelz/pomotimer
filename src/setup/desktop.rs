use std::{path::PathBuf, time::Duration};

use mde::{DesktopCategories, DesktopEntry, DesktopEntryFile, Icon};

pub fn handle_desktop(home: PathBuf) -> anyhow::Result<()> {
    let mut desktop_entry_file = home.clone();
    desktop_entry_file.push(".local/share/applications/pomotimer.desktop");

    let mut icon_file = home.clone();
    icon_file.push(".local/share/pomotimer/icon/timer.png");

    let pomotimer_bin = which::which("pomotimer")?;

    if !desktop_entry_file.exists() {
        println!("Creating desktop .entry in {:?}", desktop_entry_file);
        std::thread::sleep(Duration::from_millis(500));
        let entry = DesktopEntry::new("Pomotimer")
            .types(mde::Types::APPLICATION)
            .comment("Pomodoro Timer on Terminal")
            .exec(pomotimer_bin)
            .icon(Icon::IconPath(&icon_file))
            .terminal()
            .categories(vec![DesktopCategories::UTILITY]);

        DesktopEntryFile::new(entry).create(desktop_entry_file)?;
    }

    Ok(())
}
