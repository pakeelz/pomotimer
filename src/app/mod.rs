mod handlers;
mod notify;
mod numbers_ascii;
mod render;

use std::time::Duration;

use crossterm::event::{Event, EventStream};
use futures_util::StreamExt;
use ratatui::DefaultTerminal;

pub struct App {
    timer: i32,
    breaks: i32,
    state: State,
    need_notify: bool,
    exit: bool,
}

pub enum State {
    Timer,
    Breaks,
}

impl App {
    pub fn new(timer: i32, breaks: i32) -> Self {
        let timer = timer * 60;
        let breaks = breaks * 60;
        Self {
            timer,
            breaks,
            state: State::Timer,
            need_notify: true,
            exit: false,
        }
    }

    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        let (timer_first, breaks_first) = (self.timer, self.breaks);
        let mut reader = EventStream::new();
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        interval.tick().await;

        while !self.exit {
            self.render_ptmr(terminal)?;
            // self.render_test(terminal);

            tokio::select! {
                _ = interval.tick() => {
                    self.handle_state(timer_first, breaks_first);
                }
                Some(Ok(event)) = reader.next() => {
                    if let Event::Key(key) = event {
                        self.handle_key(key);
                    }
                }
            }
        }
        anyhow::Ok(())
    }
}

fn format_timer(sec: i32) -> String {
    let minute = sec / 60;
    let secs = sec % 60;
    format!("{minute:02}:{secs:02}")
}
