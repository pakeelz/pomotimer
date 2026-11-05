use std::env::home_dir;

use notify_rust::Notification;

use crate::app::App;

impl App {
    pub fn notify_send(&self, message: String) {
        if let Some(mut home) = home_dir() {
            home.push(".local/share/pomotimer/icon/peach.svg");
            let icon_file = home;

            if let Ok(_) = Notification::new()
                .summary("Pomotimer")
                .body(&message)
                .icon(&icon_file.to_string_lossy())
                .show()
            {}
        } else {
            if let Ok(_) = Notification::new()
                .summary("Pomotimer")
                .body(&message)
                .icon("/home/pakel/Projects/pomotimer/assets/peach.svg")
                .show()
            {}
        }
    }
}
