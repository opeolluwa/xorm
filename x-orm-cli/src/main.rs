use clap::Parser;

use crate::commands::create_database;

mod arguments;
mod commands;

// use arguments;
// use commands;

fn main() {
    let argz = arguments::Arguments::parse();
    let action = argz.action;

    // execute the desired action
    match action {
        arguments::SubCommands::Database(_) => {
            create_database();
        }
        arguments::SubCommands::Migration(_) => todo!(),
    }
    println!("Hello, world!");
}
