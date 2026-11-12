use crate::app::App;
use clap::Parser;

mod app;
mod cli;
mod setup;

#[tokio::main]
async fn main() {
    // Supress error message in tui
    let _gag = gag::Gag::stderr().unwrap();
    let cli = cli::Cli::parse();
    cli.validate_arguments();

    setup::setup().await;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    let mut app = if !cli.debug {
        App::new(cli.timer, cli.breaks)
    } else {
        App::debug_mode()
    };

    crossterm::terminal::SetTitle("Pomotimer");

    let mut terminal = ratatui::init();
    if let Err(e) = app.run(&mut terminal).await {
        tx.send(e).unwrap();
    }

    ratatui::restore();

    if let Ok(e) = rx.try_recv() {
        drop(_gag);
        eprintln!("{}", e.to_string())
    }
}
