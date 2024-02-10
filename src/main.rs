use anyhow::{Context, Result};
use clap::Parser;
use pancurses::*;

pub mod utils;
pub mod types;


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
    let mut items = get_items(&args.path)?;

    let window = initscr();
    let def_x = window.get_max_x() / 4;
    let _def_y = window.get_max_y();
    let _label_x = def_x - 20;
    // let label_y = def_y - 5;
    window.mv(0, def_x*2-10);
    window.refresh();
    let menu = vec![
        types::MenuItem {label: "[1]. Display Tasks.".to_string(), action: || println!("Displaying Tasks") },
        types::MenuItem {label: "[2]. Create New Tasks.".to_string(), action: || println!("Creating New Tasks") },
        types::MenuItem {label: "[3]. Delete Tasks.".to_string(), action: || println!("Deleting Tasks") },
        types::MenuItem {label: "[4]. Edit Tasks.".to_string(), action: || println!("Editing Tasks") },
        types::MenuItem {label: "[5]. Save.".to_string(), action: || println!("Saving Tasks") },
        types::MenuItem {label: "[-1]. Quit.".to_string(), action: || println!("Goodbye!") },
    ];

    let mut selected = 0;


    loop {
        // window.clear();
        window.mvprintw(0,0, "DOIN");
        for (i, item) in menu.iter().enumerate() {
            window.mvprintw(i as i32 + 2, 0, &item.label);
            if i == selected {
                window.attron(A_BOLD);
                window.mvprintw(i as i32 + 2, 0, &item.label);
                window.attroff(A_BOLD);
            }

        }

        window.mv(2, 0);
        window.refresh();
        window.keypad(true);
        noecho();
        match window.getch() {
            Some(Input::KeyUp) => selected = (selected + menu.len() - 1) % menu.len(),
            Some(Input::KeyDown) => selected = (selected + 1) % menu.len(),
            Some(Input::KeyDC) => break,
            Some(Input::Character(c)) => {
                if c !=  '\n'{
                    continue;
                }
                (menu[selected].action)(); 
                match selected {
                    0 => utils::display_task(&items),
                    1 => utils::create_task(&mut items),
                    2 => utils::delete_task(&mut items),
                    3 => println!("Editing Tasks"),
                    4 => utils::save_task(&items, &args),
                    _ => break,
                }
                match window.getch() {
                    _ => {
                        window.clear();
                        window.refresh();
                    }
                }
            },
            _ => {}
        }

    }
    endwin();
    Ok(())
}

