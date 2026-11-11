use clap::{CommandFactory, Parser};

#[derive(Parser)]
pub struct Cli {
    /// Timer in minutes
    #[arg(default_value_t = 30)]
    pub timer: i32,

    /// Describe break in minutes
    #[arg(default_value_t = 10)]
    pub breaks: i32,

    #[arg(hide = true, default_value_t = false, long)]
    pub debug: bool,
}

impl Cli {
    pub fn validate_arguments(&self) {
        if self.timer > 50 {
            let mut cmd = Cli::command();
            cmd.error(
                clap::error::ErrorKind::ValueValidation,
                "Timer can't exceeds 50 minute",
            )
            .exit();
        }

        if self.breaks > 15 {
            let mut cmd = Cli::command();
            cmd.error(
                clap::error::ErrorKind::ValueValidation,
                "Break can't exceeds 15 minute",
            )
            .exit();
        }
    }
}
