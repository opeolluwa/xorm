use clap::*;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
// Simplistic and asynchronous  ORM for Rust
pub struct Arguments {
    ///xorm-sub-commands
    #[clap(subcommand)]
    pub action: SubCommands,
}

/// x-orm-cli sub commands
///  "migration", "database"
#[derive(clap::Subcommand, Debug)]
pub enum SubCommands {
    ///interact with the database
    Database(DatabaseCommands),
    ///create, run and execute migrations
    Migration(MigrationsCommands),
}

/// the database subcommands are create, drop
#[derive(Args, Debug)]
pub struct DatabaseCommands {
    #[clap(subcommand)]
    pub action: DatabaseSubCommands,
}

/// database sub commands
#[derive(Debug, Subcommand)]
pub enum DatabaseSubCommands {
    ///create the database in the specified DATABASE_URL    
    Create,
    ///drop the created database
    Drop,
}

/// the migration commands
#[derive(Args, Debug)]
pub struct MigrationsCommands {
    #[clap(subcommand)]
    pub action: MigrationsSubCommands,
}
/// create the init command, essentially a struct to hold the init command arguments and options
#[derive(Subcommand, Debug)]
pub enum MigrationsSubCommands {
    /// add a new migration
    Add(MigrationName),
    /// run a migration
    Run,
    ///revert a migration
    Revert,
}

/// add migration name
#[derive(Args, Debug)]
pub struct MigrationName {
    /// the name of the migration
    #[clap(short, long, value_parser, forbid_empty_values = true)]
    pub name: String,
}
