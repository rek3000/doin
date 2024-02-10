use clap::Parser;


#[derive(Parser)]
pub struct Cli {
    pub path: std::path::PathBuf,
    #[command(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,
}

pub struct Item {
    pub id: i32,
    pub content: String,
}

pub struct MenuItem {
    pub label: String,
    pub action: fn()
}

// struct MenuItem {
//     label: String,
//     action: Box<dyn MenuItemAction>,
// }
//
// pub struct MenuAction;
//
// pub trait MenuItemAction {
//     fn display_task(items: &Vec<types::Item>);
//     fn create_task(items: &mut Vec<types::Item>);
//     fn delete_task(items: &mut Vec<types::Item>);
//     fn edit_task(items: &mut Vec<types::Item>);
//     fn save_task(items: &Vec<types::Item>, args: &types::Cli);
//     // fn display_task(&self);
//     // fn create_task(&self);
//     // fn delete_task(&self);
//     // fn edit_task(&self);
//     // fn save_task(&self);
// }
