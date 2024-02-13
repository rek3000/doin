use std::io::stdout;
use anyhow::Result;

use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use std::any::type_name;

pub mod app;
pub mod types;
pub mod utils;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn main() -> Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let _app = app::App::new().run();
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
