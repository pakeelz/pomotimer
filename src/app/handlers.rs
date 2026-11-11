use crossterm::event::{KeyCode, KeyEvent};
use tray_icon::TrayIcon;

use crate::app::{
    App, State, format_timer_tray,
    tray::{IconState, load_icon},
};

impl App {
    pub fn handle_state_with_tray(&mut self, tray: &mut TrayIcon) {
        match self.state {
            State::Timer => {
                if self.need_notify {
                    self.notify_send_cmd(format!("Timer set in {} minutes", self.cache.timer / 60));
                    self.need_notify = false;
                }
                if self.timer == 0 {
                    self.state = State::Breaks;
                    self.timer = self.cache.timer;
                    self.need_notify = true;
                }
                self.timer -= 1;

                if let Ok(icon) = load_icon(IconState::Timer) {
                    if let Ok(_) = tray.set_icon(Some(icon)) {
                        tray.set_title(Some(format_timer_tray(self.timer)));
                    }
                }
            }
            State::Breaks => {
                if self.need_notify {
                    self.notify_send_cmd(format!(
                        "Let's take a break for {} minutes",
                        self.cache.breaks / 60
                    ));
                    self.need_notify = false;
                }
                if self.breaks == 0 {
                    self.state = State::Timer;
                    self.breaks = self.cache.breaks;
                    self.need_notify = true;
                }
                self.breaks -= 1;
                if let Ok(icon) = load_icon(IconState::Breaks) {
                    if let Ok(_) = tray.set_icon(Some(icon)) {
                        tray.set_title(Some(format_timer_tray(self.breaks)));
                    }
                }
            }
        }
    }

    pub fn handle_state(&mut self) {
        match self.state {
            State::Timer => {
                if self.need_notify {
                    self.notify_send_cmd(format!("Timer set in {} minutes", self.cache.timer / 60));
                    self.need_notify = false;
                }
                if self.timer == 0 {
                    self.state = State::Breaks;
                    self.timer = self.cache.timer;
                    self.need_notify = true;
                }
                self.timer -= 1;
            }
            State::Breaks => {
                if self.need_notify {
                    self.notify_send_cmd(format!(
                        "Let's take a break for {} minutes",
                        self.cache.breaks / 60
                    ));
                    self.need_notify = false;
                }
                if self.breaks == 0 {
                    self.state = State::Timer;
                    self.breaks = self.cache.breaks;
                    self.need_notify = true;
                }
                self.breaks -= 1;
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Enter if self.is_breaks() => {
                self.state = State::Timer;
                self.timer = self.cache.timer;
                self.need_notify = true;
            }
            _ => {}
        }
    }
}
