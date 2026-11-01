use notify_rust::Notification;

use crate::app::App;

impl App {
    #[allow(dead_code)]
    pub fn notify_send(&self, message: String) {
        if let Ok(_) = Notification::new()
            .summary("Pomotimer")
            .body(&message)
            .icon("/home/pakel/Projects/pomotimer/assets/peach.svg")
            .show()
        {}
    }
}
