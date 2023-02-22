use clap::Parser;

use crate::commands::{
    add_migrations, create_database, drop_database, revert_migrations, run_migrations,
};

mod arguments;
mod commands;

fn main() {
    let argz = arguments::Arguments::parse();
    let action = argz.action;

    // execute the desired action
    match action {
        arguments::SubCommands::Database(commands) => match commands.action {
            arguments::DatabaseSubCommands::Create => create_database(),
            arguments::DatabaseSubCommands::Drop => drop_database(),
        },
        arguments::SubCommands::Migration(commands) => match commands.action {
            arguments::MigrationsSubCommands::Add(migration_name) => {
                add_migrations(migration_name.name)
            }
            arguments::MigrationsSubCommands::Run => run_migrations(),
            arguments::MigrationsSubCommands::Revert => revert_migrations(),
        },
    }
}
