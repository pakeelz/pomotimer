use ratatui::{
    DefaultTerminal,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    text::Line,
    widgets::{Block, BorderType, Borders, Paragraph},
};

use crate::app::{
    self, App, State,
    numbers_ascii::{SEPARATOR, convert_to_assci},
};

impl App {
    pub fn render_ptmr(&self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        let (cols, rows) = crossterm::terminal::size()?;
        terminal.draw(|frame| {
            let (headers, timer) = match self.state {
                State::Timer => (" Timer ", app::format_timer(self.timer)),
                State::Breaks => (" Break ", app::format_timer(self.breaks)),
            };

            let (width, height) = (51, 11);
            let x = cols.saturating_sub(width) / 2;
            let y = rows.saturating_sub(height) / 2;

            let rect = Rect::new(x, y, width, height);
            let [header, body] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(5), Constraint::Fill(1)])
                .areas(rect);

            let [_space, body, footer] = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Fill(1),
                    Constraint::Length(2),
                ])
                .areas(body);

            let content = concat!(
                "████   ███  █   █  ███  █████ ███ █   █ █████ ████ \n",
                "█   █ █   █ ██ ██ █   █   █    █  ██ ██ █     █   █\n",
                "████  █   █ █ █ █ █   █   █    █  █ █ █ ████  ████ \n",
                "█     █   █ █   █ █   █   █    █  █   █ █     █  █ \n",
                "█      ███  █   █  ███    █   ███ █   █ █████ █   █\n"
            );

            let header_content = Paragraph::new(content).centered().fg(Color::Yellow);
            let body_content = Paragraph::new(timer)
                .fg(Color::Yellow)
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .title_top(headers)
                        .title_alignment(ratatui::layout::HorizontalAlignment::Center),
                )
                .centered();

            let footer_content_text = match self.state {
                State::Timer => vec![Line::from("Press q to give up")],
                State::Breaks => vec![
                    Line::from("Press q to give up"),
                    Line::from("Press Enter to skip break"),
                ],
            };

            let footer_content = Paragraph::new(footer_content_text).centered().red().bold();

            frame.render_widget(header_content, header);
            frame.render_widget(body_content, body);
            frame.render_widget(footer_content, footer);
        })?;
        Ok(())
    }

    #[allow(unused)]
    pub fn render_test(&self, terminal: &mut DefaultTerminal) {
        let (cols, rows) = crossterm::terminal::size().unwrap();
        terminal
            .draw(|frame| {
                let (width, height) = (51, 20);
                let x = cols.saturating_sub(width) / 2;
                let y = rows.saturating_sub(height) / 2;

                let rect = Rect::new(x, y, width, height);
                let [top, bottom] = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Length(7), Constraint::Fill(1)])
                    .areas(rect);

                let [
                    _spacer,
                    left,
                    left_center,
                    separator,
                    right_center,
                    right,
                    _spacer2,
                ] = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(25),
                        Constraint::Length(5),
                        Constraint::Length(5),
                        Constraint::Length(5),
                        Constraint::Length(5),
                        Constraint::Length(5),
                        Constraint::Percentage(25),
                    ])
                    .areas(bottom);

                let timer = app::format_timer(self.timer);

                let content = concat!(
                    "████   ███  █   █  ███  █████ ███ █   █ █████ ████ \n",
                    "█   █ █   █ ██ ██ █   █   █    █  ██ ██ █     █   █\n",
                    "████  █   █ █ █ █ █   █   █    █  █ █ █ ████  ████ \n",
                    "█     █   █ █   █ █   █   █    █  █   █ █     █  █ \n",
                    "█      ███  █   █  ███    █   ███ █   █ █████ █   █\n"
                );

                let background = Paragraph::new("").bg(Color::from_u32(0xff1c2121));
                frame.render_widget(background, frame.area());

                let par = Paragraph::new(content)
                    .centered()
                    .fg(Color::Yellow)
                    .block(Block::new().borders(Borders::BOTTOM));
                frame.render_widget(par, top);

                for (index, char) in timer.char_indices() {
                    match index {
                        0 => {
                            let content = convert_to_assci(&char);
                            frame.render_widget(content, left);
                        }
                        1 => {
                            let content = convert_to_assci(&char);
                            frame.render_widget(content, left_center);
                        }
                        2 => {
                            let sep = Paragraph::new(SEPARATOR).fg(Color::Red).centered();
                            frame.render_widget(sep, separator);
                        }
                        3 => {
                            let content = convert_to_assci(&char);
                            frame.render_widget(content, right_center);
                        }
                        4 => {
                            let content = convert_to_assci(&char);
                            frame.render_widget(content, right);
                        }
                        _ => {}
                    }
                }
            })
            .unwrap();
    }
}
