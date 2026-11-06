use crossterm::event::{KeyCode, KeyEvent};

use crate::app::{App, State};

impl App {
    pub fn handle_state(&mut self, timer_first: i32, breaks_first: i32) {
        match self.state {
            State::Timer => {
                if self.need_notify {
                    self.notify_send_cmd(format!("Timer set in {} minutes", self.timer / 60));
                    self.need_notify = false;
                }
                if self.timer == 0 {
                    self.state = State::Breaks;
                    self.timer = timer_first;
                    self.need_notify = true;
                }
                self.timer -= 1;
            }
            State::Breaks => {
                if self.need_notify {
                    self.notify_send_cmd(format!(
                        "Let's take a break for {} minutes",
                        self.breaks / 60
                    ));
                    self.need_notify = false;
                }
                if self.breaks == 0 {
                    self.state = State::Timer;
                    self.breaks = breaks_first;
                    self.need_notify = true;
                }
                self.breaks -= 1;
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            _ => {}
        }
    }
}
