use itertools::Itertools;
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
    state: RunningState,
    message: Message,
}

impl App {
    pub fn new() -> Self {
        App {
            task_selected: 0,
            items: vec![],
            state: RunningState::Running,
            message: Message::None,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

        let args = Cli::parse();
        while self.state != RunningState::Done {
            let _ = self.get_items(&args.path).unwrap();
            let _ = self.draw(&mut terminal);
            let _ = self.handle_event();
            let _ = self.update();
        }
        Ok(())
    }

    fn draw(&self, terminal: &mut Terminal<impl Backend>) -> Result<()> {
        terminal.draw(|frame| {
            frame.render_widget(self, frame.size());
        })?;

        Ok(())
    }

    fn get_items(&mut self, path: &std::path::PathBuf) -> Result<()> {
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

        self.items = items;
        Ok(())
    }

    fn handle_event(&mut self) -> Result<Option<Message>> {
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                let msg = self.handle_key(key);
                self.message = msg.unwrap();
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
            _ => Some(Message::None),
        }
    }

    fn update(&mut self) -> Option<Message> {
        match self.message {
            Message::Add => {}
            Message::Edit => {}
            Message::MoveUp => {
                if (self.items.len() != 0) && self.task_selected != 0 {
                    self.task_selected -= 1;
                }
                self.message = Message::None;
            }
            Message::MoveDown => {
                if !((self.items.len() == 0) || self.task_selected == (self.items.len() - 1)) {
                    self.task_selected += 1;
                }
                self.message = Message::None;
            }
            Message::Delete => {}
            Message::Quit => {
                self.state = RunningState::Done;
            }
            Message::None => {}
        };
        None
    }

    fn render_task(&self, area: Rect, buf: &mut Buffer) -> io::Result<()> {
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
        par.render(task_area, buf);
        des.render(des_area, buf);
        Ok(())
    }

    fn render_option(&self, area: Rect, buf: &mut Buffer) -> io::Result<()> {
        let options = [
            ("Q/ESC", "Quit"),
            ("↑", "Up"),
            ("↓", "Down"),
            ("A/a", "Add Task"),
            ("E/e", "Edit Task"),
            ("D/d", "Delete Task"),
        ];

        let spans = options
            .iter()
            .flat_map(|(key, desc)| {
                let key = Span::styled(format!(" {} ", key), Style::new().black().on_gray());
                let desc = Span::styled(format!(" {} ", desc), Style::new().gray().on_black());

                [key, desc]
            })
            .collect_vec();

        let line = Line::from(spans).centered();
        line.render(area, buf);
        Ok(())
    }

    fn centered_rect(&self, percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
        // Cut the given rectangle into three vertical pieces
        let popup_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ])
            .split(rect);

        // Then cut the middle vertical piece into three width-wise pieces
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ])
            .split(popup_layout[1])[1] // Return the middle chunk
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let main_block = Block::default()
            .title("DOIN - Slow Task Management App".bold().cyan())
            .borders(Borders::ALL)
            .on_black();
        let main_layout = Layout::new(
            Direction::Vertical,
            [Constraint::Min(0), Constraint::Length(1)],
        )
        .split(area);

        main_block.render(main_layout[0], buf);
        // Render Inner Layout
        let _ = self.render_task(main_layout[0], buf);
        let _ = self.render_option(main_layout[1], buf);

        match self.message {
            Message::Add => {
                let popup_area = self.centered_rect(area.width - 20, area.height, area);
                let add_layout = Layout::new(
                    Direction::Vertical,
                    [Constraint::Percentage(30), Constraint::Percentage(70)],
                )
                .split(popup_area);
                let popup = Block::default()
                    .title("Add a Task".red())
                    .borders(Borders::ALL)
                    .style(Style::new().black().on_gray());

                Clear.render(popup_area, buf);
                let add_title = Block::default()
                    .title("Title")
                    .borders(Borders::ALL)
                    .style(Style::new().black().on_gray());
                let add_desc = Block::default()
                    .title("Description")
                    .borders(Borders::ALL)
                    .style(Style::new().black().on_gray());
                let add_title_area = add_title.inner(add_layout[0]);
                let add_desc_area = add_desc.inner(add_layout[1]);
                popup.render(popup_area, buf);
                add_title.render(add_title_area, buf);
                add_desc.render(add_desc_area, buf);
            }
            Message::Edit => {}
            Message::Delete => {}
            _ => {}
        }
    }
}
