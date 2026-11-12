mod handlers;
mod notify;
mod numbers_ascii;
mod render;
mod tray;

use std::time::Duration;

use crossterm::event::{Event, EventStream};
use futures_util::StreamExt;
use ratatui::DefaultTerminal;

use crate::app::tray::{IconState, register_tray};

pub struct App {
    timer: i32,
    breaks: i32,
    cache: Cache,
    state: State,
    need_notify: bool,
    exit: bool,
}

pub enum State {
    Timer,
    Breaks,
}

pub struct Cache {
    timer: i32,
    breaks: i32,
}

#[allow(dead_code)]
impl App {
    pub fn new(timer: i32, breaks: i32) -> Self {
        let timer = timer * 60;
        let breaks = breaks * 60;
        let cache = Cache { timer, breaks };
        Self {
            timer,
            breaks,
            cache,
            state: State::Timer,
            need_notify: true,
            exit: false,
        }
    }

    pub fn debug_mode() -> Self {
        let timer = 15;
        let breaks = 15;
        let cache = Cache { timer, breaks };

        Self {
            timer,
            breaks,
            cache,
            state: State::Breaks,
            need_notify: true,
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        let mut reader = EventStream::new();
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        interval.tick().await;

        if let Ok(mut tray) = register_tray(IconState::Timer) {
            while !self.exit {
                self.render_ptmr(terminal)?;

                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_millis(20)) => {
                        while gtk::events_pending() {
                            gtk::main_iteration_do(false);
                        }
                    }
                    _ = interval.tick() => {
                        self.handle_state_with_tray(&mut tray);
                    }
                    Some(Ok(event)) = reader.next() => {
                        if let Event::Key(key) = event {
                            self.handle_key(key);
                        }
                    }
                }
            }
        } else {
            while !self.exit {
                self.render_ptmr(terminal)?;

                tokio::select! {
                    _ = tokio::time::sleep(Duration::from_millis(20)) => {
                        while gtk::events_pending() {
                            gtk::main_iteration_do(false);
                        }
                    }
                    _ = interval.tick() => {
                        self.handle_state();
                    }
                    Some(Ok(event)) = reader.next() => {
                        if let Event::Key(key) = event {
                            self.handle_key(key);
                        }
                    }
                }
            }
        }
        anyhow::Ok(())
    }

    pub fn is_breaks(&self) -> bool {
        match self.state {
            State::Timer => false,
            State::Breaks => true,
        }
    }
}

fn format_timer(sec: i32) -> String {
    let minute = sec / 60;
    let secs = sec % 60;
    format!("{minute:02}:{secs:02}")
}

fn format_timer_tray(sec: i32) -> String {
    let minute = sec / 60;
    let secs = sec % 60;
    if minute <= 0 {
        format!("{secs}")
    } else {
        format!("{minute:02}:{secs:02}")
    }
}
