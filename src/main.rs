use anyhow::{Context, Result};
use clap::Parser;
use std::io;
use std::io::Write;

pub mod utils;
pub mod types;


fn main() -> Result<()> {
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
        println!("-------DOIN--------");
        println!("[1]. Display Tasks.");
        println!("[2]. Create New Tasks.");
        println!("[3]. Delete Tasks.");
        println!("[4]. Edit Tasks.");
        println!("[5]. Save.");
        println!("[-1]. Quit.");
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

