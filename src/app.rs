use std::io;
use std::io::stdout;
use std::rc::Rc;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};

use crate::types;

pub struct App {
    task_selected: i32,
    items: Vec<types::Item>,
}

impl App {
    pub fn new() -> App {
        App {
            task_selected: 0,
            items: vec![],
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let mut quit = false;
        while !quit {
            let args = types::Cli::parse();
            self.items = self.get_items(&args.path).unwrap();
            terminal.draw(|frame| self.ui(frame))?;
            quit = self.handle_event()?;
        }
        Ok(())
    }

    fn get_items(&self, path: &std::path::PathBuf) -> Result<Vec<types::Item>, anyhow::Error> {
        let mut items: Vec<types::Item> = Vec::new();

        // Read file to String
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read file `{}`", path.display()))?;

        let mut index = 0;
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

    fn handle_event(&mut self) -> io::Result<bool> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    return Ok(true);
                } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Up {
                    if self.task_selected != 0 {
                        self.task_selected -= 1;
                    }
                } else if key.kind == event::KeyEventKind::Press && key.code == KeyCode::Down {
                    if self.task_selected == (self.items.len() - 1).try_into().unwrap() {
                    } else {
                        self.task_selected += 1;
                    }
                }
            }
        }

        Ok(false)
    }

    fn ui(&self, frame: &mut Frame) {
        // Initialize
        let main_layout = self.render_main(frame);
        // Render Inner Layout
        let _ = self.render_task(frame, main_layout[1]);
        let _ = self.render_option(frame, main_layout[2]);
    }

    fn render_task(&self, frame: &mut Frame, area: Rect) -> io::Result<()> {
        let mut tasks: Vec<Line> = vec![];
        // for item in self.items {
        for item in &self.items {
            if self.task_selected == item.id {
                tasks.push(Line::from(
                    Span::from(item.content.clone()).style(Style::new().black().on_gray()),
                ));
            } else {
                tasks.push(Line::from(item.content.clone()));
            }
        }

        let task_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(80), Constraint::Percentage(20)],
        )
        .split(area);

        let par = Paragraph::new(tasks)
            .block(
                Block::new()
                    .title("Tasks".italic().green())
                    .borders(Borders::RIGHT | Borders::LEFT | Borders::BOTTOM),
            )
            .style(Style::new().on_black());
        let des = Block::default()
            .title("Description".italic().green())
            .borders(Borders::RIGHT | Borders::LEFT | Borders::BOTTOM)
            .on_black();

        frame.render_widget(par, task_layout[0]);
        frame.render_widget(des, task_layout[1]);
        Ok(())
    }

    fn render_option(&self, frame: &mut Frame, area: Rect) -> io::Result<()> {
        let options = [
            ("Q/ESC", "Quit"),
            ("↑", "Up"),
            ("↓", "Down"),
            ("A/a", "Add Task"),
            ("E/e", "Edit Task"),
            ("D/d", "Delete Task"),
        ];

        let option_layout = Layout::new(
            Direction::Horizontal,
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
                .iter()
                .map(|&c| Constraint::Min(c)),
        )
        .split(area);

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

    fn render_main(&self, frame: &mut Frame) -> Rc<[Rect]> {
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
}
