use std::io;
use std::io::stdout;
use std::rc::Rc;

use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{self, Event, KeyCode};
use ratatui::{prelude::*, widgets::*};

use crate::types::*;

#[derive(Default)]
pub struct App {
    task_selected: usize,
    items: Vec<Item>,
    // quit: bool,
    state: RunningState,
}

impl App {
    pub fn new() -> App {
        App {
            task_selected: 0,
            items: vec![],
            state: RunningState::Running,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let args = Cli::parse();
        while self.state != RunningState::Done {
            self.items = self.get_items(&args.path).unwrap();
            terminal.draw(|frame| self.ui(frame))?;
            let mut cur_msg = self.handle_event().unwrap();
            while cur_msg.is_some() {
                cur_msg = self.update(cur_msg.unwrap());

            }
        }
        Ok(())
    }

    fn get_items(&self, path: &std::path::PathBuf) -> Result<Vec<Item>, anyhow::Error> {
        let mut items: Vec<Item> = Vec::new();

        // Read file to String
        let data = std::fs::read_to_string(path)
            .with_context(|| format!("Could not read file `{}`", path.display()))?;
        let data_items = json::parse(&data);
        for data_item in data_items.unwrap().members() {
            let item: Item = Item {
                id: data_item["id"].as_usize().unwrap(),
                title: data_item["title"].as_str().unwrap().to_string(),
                content: data_item["content"].as_str().unwrap().to_string(),
            };
            items.push(item);
        }
        return Ok(items);
    }

    fn handle_event(&mut self) -> Result<Option<Message>> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                return Ok(self.handle_key(key));
            }
        }
        Ok(None)
    }

    fn handle_key(&self, key: event::KeyEvent) -> Option<Message> {
        match key.code {
            KeyCode::Char('q') => Some(Message::Quit),
            KeyCode::Up => Some(Message::MoveUp),
            KeyCode::Down => Some(Message::MoveDown),
            KeyCode::Char('a') => Some(Message::Add),
            KeyCode::Char('e') => Some(Message::Edit),
            KeyCode::Char('d') => Some(Message::Delete),
            _ => None,
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        match msg {
            Message::Add => {}
            Message::Edit => {}
            Message::MoveUp => {
                if (self.items.len() != 0) && self.task_selected != 0 {
                    self.task_selected -= 1;
                }
            }
            Message::MoveDown => {
                if !((self.items.len() == 0) || self.task_selected == (self.items.len() - 1)) {
                    self.task_selected += 1;
                }
            }
            Message::Delete => {}
            Message::Quit => {
                self.state = RunningState::Done;
            }
        };
        None
    }
    fn ui(&self, frame: &mut Frame) {
        // Initialize
        let main_layout = self.render_main(frame);
        // Render Inner Layout
        let _ = self.render_task(frame, main_layout[0]);
        let _ = self.render_option(frame, main_layout[1]);
    }

    fn render_task(&self, frame: &mut Frame, area: Rect) -> io::Result<()> {
        let mut tasks: Vec<Line> = vec![];
        for item in &self.items {
            if self.task_selected == item.id {
                tasks.push(Line::from(
                    Span::from(item.title.clone()).style(Style::new().black().on_gray()),
                ));
            } else {
                tasks.push(Line::from(item.title.clone()));
            }
        }

        let des_text = Text::from((&self.items[self.task_selected].content).clone());

        let task_layout = Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(60), Constraint::Percentage(40)],
        )
        .horizontal_margin(1)
        .split(area);

        let par_block = Block::new()
            .title("Tasks".italic().green())
            .borders(Borders::ALL);
        let par = Paragraph::new(tasks)
            .block(par_block.clone())
            .style(Style::new().on_black());
        let des_block = Block::default()
            .title("Description".italic().green())
            .borders(Borders::ALL)
            .on_black();
        let des = Paragraph::new(des_text)
            .block(des_block.clone())
            .style(Style::new().on_black());

        let task_area = par_block.inner(task_layout[0]);
        let des_area = des_block.inner(task_layout[1]);
        frame.render_widget(par, task_area);
        frame.render_widget(des, des_area);
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
        .horizontal_margin(0)
        .split(area);

        let mut index = 0;
        for (label, info) in options {
            let btn = Block::default()
                .title(label)
                .title_alignment(Alignment::Center)
                .black()
                .on_gray();
            let des = Block::default()
                .title(info)
                .title_alignment(Alignment::Center)
                .gray()
                .on_black();
            frame.render_widget(btn, option_layout[index]);
            index += 1;
            frame.render_widget(des, option_layout[index]);
            index += 1;
        }

        Ok(())
    }

    fn render_main(&self, frame: &mut Frame) -> Rc<[Rect]> {
        let main_block = Block::default()
            .title("DOIN - Slow Task Management App".bold().cyan())
            .borders(Borders::ALL)
            .on_black();
        let main_area = frame.size();
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(1)],
        )
        .split(main_area);
        frame.render_widget(main_block, main_layout[0]);
        main_layout
    }

    fn create(&self, frame: &mut Frame, area: Rect) {
        let popup = Block::default().title("Add Task");
        let popup_area = frame.size();
    }
    fn edit(&self, frame: &mut Frame, area: Rect) {}
    fn delete(&self, frame: &mut Frame, area: Rect) {}
    fn save(&self, frame: &mut Frame, area: Rect) {}
}
