use crate::types;
use std::io;
use std::io::Write;

pub fn display_task(items: &Vec<types::Item>) {
    println!("+---------------------------+");
    println!("|----------TASKS------------|");
    for line in items {
        println!("|[{}]. {}", line.id, line.content);
    }
    println!("+---------------------------+");
}

pub fn create_task(items: &mut Vec<types::Item>) {
    let mut input = String::new();
    loop {
        print!("Number of Tasks: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read number");

        let number: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!();
                continue;
            }
        };

        for i in 0..number {
            let mut content = String::new();
            print!("Task{i}: ");
            io::stdout().flush().unwrap();
            io::stdin()
                .read_line(&mut content)
                .expect("Failed to read number");

            let content = String::from(content.trim());
            let item = types::Item {
                id: items[items.len() -1].id + 1,
                content,
            };

            items.push(item);
        }
        break;
    }
}

pub fn delete_task(items: &mut Vec<types::Item>) {
    loop {
        let mut input = String::new();
        print!("Task Number: ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read number");

        let number: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!();
                continue;
            }
        };

        if number > items.len() {
            println!("~please choose correct number~");
            continue;
        } else {
            items.remove(number - 1);
            println!("Deleted task number {number}");
            break;
        }
    }

}
