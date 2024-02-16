use clap::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
    // // #[command(flatten)]
    // pub verbose: clap_verbosity_flag::Verbosity,
}


// #[derive(Hash)]
pub struct Item {
    pub id: usize,
    pub title: String,
    pub content: String,
}

pub struct MenuItem {
    pub label: String,
    pub action: fn()
}
