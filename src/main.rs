use std::io;
use std::io::Write;

use anyhow::{Context, Result};
use clap::Parser;
use pancurses::{initscr, Input, noecho, endwin};

pub mod utils;
pub mod types;


fn main() -> Result<()> {
    let window = initscr();
    let def_x = window.get_max_x() / 4;
    let def_y = window.get_max_y();
    let label_x = def_x - 20;
    // let label_y = def_y - 5;
    window.mv(0, def_x*2-10);
    window.refresh();
    window.printw("+----------DOIN----------+");
    window.mv(window.get_cur_y()+1, def_x*2 - 10);
    window.addstr("| [1]. Display Tasks.    |");
    window.mv(window.get_cur_y()+1, def_x*2 - 10);
    window.addstr("| [2]. Create New Tasks. |");
    window.mv(window.get_cur_y()+1, def_x*2 - 10);
    window.addstr("| [3]. Delete Tasks.     |");
    window.mv(window.get_cur_y()+1, def_x*2 - 10);
    window.addstr("| [4]. Edit Tasks.       |");
    window.mv(window.get_cur_y()+1, def_x*2 - 10);
    window.addstr("| [5]. Save.             |");
    window.mv(window.get_cur_y()+1, def_x*2 - 10);
    window.addstr("| [-1]. Quit.            |");
    window.mv(window.get_cur_y()+1, def_x*2 - 10);
    window.addstr("+------------------------+");
    window.mv(window.get_cur_y()+1, label_x);

    window.refresh();
    window.addstr("ENTER CHOICE: ");
    window.mv(window.get_cur_y(), def_x);
    window.refresh();
    window.keypad(true);
    noecho();
    let mut input_str = String::new();
    loop {
        match window.getch() {
            Some(Input::Character(c)) => { 
                window.insch(c);
                window.mv(window.get_cur_y(), window.get_cur_x() + 1);
                input_str.push(c);
            },
            Some(Input::KeyBackspace) => { 
                if (input_str.len() == 0) || (window.get_cur_x() == def_x) {
                    continue;
                }
                window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                window.delch();
                let index: usize = (window.get_cur_x() - def_x).try_into().unwrap();
                input_str.remove(index);
            },
            Some(Input::KeyLeft) => { 
                let index: usize = (window.get_cur_x() - def_x).try_into().unwrap();
                if index >= 1 {
                    window.mv(window.get_cur_y(), window.get_cur_x() - 1);
                } 
            },
            Some(Input::KeyRight) => { 
                let index: usize = (window.get_cur_x() - def_x).try_into().unwrap();
                if index < input_str.len().try_into().unwrap() {
                    window.mv(window.get_cur_y(), window.get_cur_x() + 1);
                }
            },
            Some(Input::KeyUp) => { 
                window.mv(window.get_cur_y() - 1, window.get_cur_x());
            },
            Some(Input::KeyDown) => { 
                window.mv(window.get_cur_y() + 1, window.get_cur_x());
            },
            Some(Input::KeyDC) => break,
            _ => (),
        }
        window.refresh();
    }
    endwin();
    println!("{}", input_str);
    let mut items: Vec<types::Item> = Vec::new();
    let args = types::Cli::parse();

    // Read file to String
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("Could not read file `{}`", args.path.display()))?;

    let mut index = 1;
    for line in content.lines() {
        let item = types::Item {
            id: index,
            content: String::from(line),
        };
        items.push(item);
        index += 1;
    }
    loop {
        let mut choice = String::new();
        println!("+----------DOIN----------+");
        println!("| [1]. Display Tasks.    |");
        println!("| [2]. Create New Tasks. |");
        println!("| [3]. Delete Tasks.     |");
        println!("| [4]. Edit Tasks.       |");
        println!("| [5]. Save.             |");
        println!("| [-1]. Quit.            |");
        println!("+------------------------+");

        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!();
                continue;
            },
        };

        match choice {
            1 => utils::display_task(&items),
            2 => utils::create_task(&mut items),
            3 => utils::delete_task(&mut items),
            4 => println!("Editing Tasks"),
            5 => utils::save_task(&items, &args),
            _ => { 
                println!("Goodbye!");
                break;
            },
        }
        println!();
    }

    Ok(())
}

