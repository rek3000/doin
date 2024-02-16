use clap::Parser;


#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

pub struct Item {
    pub id: usize,
    pub title: String,
    pub content: String,
}

pub struct MenuItem {
    pub label: String,
    pub action: fn()
}
