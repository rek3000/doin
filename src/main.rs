use std::io::{self, stdout};
use std::rc::Rc;

use anyhow::{Context, Result};
use clap::Parser;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use ratatui::{prelude::*, widgets::*};

use std::any::type_name;

pub mod types;
pub mod utils;

fn type_of<T>(_: &T) -> &'static str {
    type_name::<T>()
}

fn get_items(path: &std::path::PathBuf) -> Result<Vec<types::Item>, anyhow::Error> {
    let mut items: Vec<types::Item> = Vec::new();

    // Read file to String
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;

    let mut index = 1;
    for line in content.lines() {
        let item = types::Item {
            id: index,
            content: String::from(line),
        };
        items.push(item);
        index += 1;
    }
    return Ok(items);
}

fn main() -> Result<()> {
    let args = types::Cli::parse();
    let mut _items = get_items(&args.path)?;
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut quit = false;
    while !quit {
        terminal.draw(ui)?;
        quit = handle_event()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn handle_event() -> io::Result<bool> {
    if event::poll(std::time::Duration::from_millis(50))? {
        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                return Ok(true);
            }
        }
    }

    Ok(false)
}

fn render_task(frame: &mut Frame, area: Rect) -> io::Result<()> {
    let task_layout = Layout::new(
        Direction::Horizontal,
        [Constraint::Percentage(80), Constraint::Percentage(20)],
        )
        .split(area);
    let tasks = vec![
        Line::from("Writing CLI Tools"),
        Line::from("Reading Philosophy Books"),
        Line::from("Do Bed"),
    ];

    let par = Paragraph::new(tasks)
        .block(
            Block::new()
                .title("Tasks".italic().green())
                .borders(Borders::RIGHT | Borders::LEFT | Borders::BOTTOM),
        )
        .style(Style::new().on_black());
    let des = Block::default()
        .title("description".italic().green())
        .borders(Borders::RIGHT | Borders::LEFT | Borders::BOTTOM)
        .on_black();


    frame.render_widget(par, task_layout[0]);
    frame.render_widget(des, task_layout[1]);
    Ok(())
}

// fn render_des(frame: &mut Frame, area: Rect) -> io::Result<()> {
//     let des = block::default()
//         .title("description".italic().green())
//         .borders(borders::right | borders::left | borders::bottom)
//         .on_black();
//     frame.render_widget(des, area);
//     Ok(())
// }

fn render_option(frame: &mut Frame, area: Rect) -> io::Result<()> {
    let option_layout = Layout::new(
        Direction::Horizontal,
        [
            // Quit
            Constraint::Min(1),
            Constraint::Min(1),
            // Up
            Constraint::Min(1),
            Constraint::Min(1),
            // Down
            Constraint::Min(1),
            Constraint::Min(1),
            // Add
            Constraint::Min(1),
            Constraint::Min(1),
            // Edit
            Constraint::Min(1),
            Constraint::Min(1),
            // Delete
            Constraint::Min(1),
            Constraint::Min(1),
        ],
    )
    .split(area);

    let options = [
        ("Q/ESC", "Quit"),
        ("↑", "Up"),
        ("↓", "Down"),
        ("A/a", "Add Task"),
        ("E/e", "Edit Task"),
        ("D/d", "Delete Task"),
    ];
    let mut index = 0;
    for (label, info) in options {
        let btn = Block::default()
            .title(label)
            .title_alignment(Alignment::Center)
            .padding(Padding::horizontal(1))
            .black()
            .on_gray();
        let des = Block::default()
            .title(info)
            .title_alignment(Alignment::Center)
            .padding(Padding::horizontal(1))
            .on_black();
        frame.render_widget(btn, option_layout[index]);
        index += 1;
        frame.render_widget(des, option_layout[index]);
        index += 1;
    }

    Ok(())
}

fn render_main(frame: &mut Frame) -> Rc<[Rect]>{
    let main_layout = Layout::new(
        Direction::Vertical,
        [
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ],
    )
    .split(frame.size());
    frame.render_widget(
        Block::default()
            .title("DOIN".bold().cyan())
            .borders(Borders::ALL)
            .padding(Padding::new(40, 40, 20, 20))
            .on_black(),
        main_layout[0],
    );
    main_layout
} 

fn ui(frame: &mut Frame) {
    // Initialize
    let main_layout = render_main(frame);
    // Render Inner Layout
    let _ = render_task(frame, main_layout[1]);
    let _ = render_option(frame, main_layout[2]);
}
