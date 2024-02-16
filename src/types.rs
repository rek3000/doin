use clap::Parser;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
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

#[derive(Default, PartialEq)]
pub enum RunningState {
    #[default]
    Running,
    Done,
}

pub enum Message {
    MoveUp,
    MoveDown,
    Add,
    Edit,
    Delete,
    Quit,
}
