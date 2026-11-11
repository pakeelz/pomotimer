use std::{env::home_dir, path::PathBuf, process::Command};

use crate::app::App;

impl App {
    #[cfg(target_os = "linux")]
    pub fn notify_send_cmd(&self, message: String) {
        use std::path::PathBuf;

        let home_dir = home_dir().unwrap_or_else(|| PathBuf::from("."));
        let mut sound_path = home_dir;
        sound_path.push(".local/share/pomotimer/sounds");

        let summary = match self.state {
            super::State::Timer => {
                sound_path.push("clink.mp3");
                "Timer"
            }
            super::State::Breaks => {
                sound_path.push("bell.mp3");
                "Breaks"
            }
        };

        if let Ok(_) = Command::new("notify-send")
            .args([
                "-a",
                "Pomotimer",
                "-e",
                "-n",
                "/home/pakel/.local/share/pomotimer/icon/timer.png",
                summary,
                &message,
            ])
            .spawn()
        {
            if let Ok(_) = play(sound_path) {}
        }
    }
}

fn play(path_sound: PathBuf) -> anyhow::Result<()> {
    if let Err(_) = Command::new("paplay").arg(&path_sound).spawn() {
        let path_sound = path_sound.to_str().unwrap_or_else(|| "");
        Command::new("pw-cat").args(["-p", path_sound]).spawn()?;
    }
    anyhow::Ok(())
}
