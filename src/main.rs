use crate::app::App;
use clap::Parser;

mod app;
mod cli;

#[tokio::main]
async fn main() {
    let cli = cli::Cli::parse();
    cli.validate_arguments();

    let mut app = App::new(cli.timer, cli.breaks);
    let mut terminal = ratatui::init();
    if let Ok(_) = app.run(&mut terminal).await {}

    ratatui::restore();
}
