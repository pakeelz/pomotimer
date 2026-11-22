use std::path::PathBuf;

use mde::{DesktopCategories, DesktopEntry, DesktopEntryFile, Icon};

pub fn handle_desktop(home: PathBuf) -> anyhow::Result<()> {
    let mut desktop_entry_file = home.clone();
    desktop_entry_file.push(".local/share/applications/pomotimer.desktop");

    let mut icon_file = home.clone();
    icon_file.push(".local/share/pomotimer/icon/timer.png");

    if !desktop_entry_file.exists() {
        let entry = DesktopEntry::new("Pomotimer")
            .types(mde::Types::APPLICATION)
            .comment("Pomodoro Timer on Terminal")
            .exec("/usr/local/bin/pomotimer")
            .icon(Icon::IconPath(&icon_file))
            .terminal()
            .categories(vec![DesktopCategories::UTILITY]);

        DesktopEntryFile::new(entry).create(desktop_entry_file)?;
    }

    Ok(())
}
