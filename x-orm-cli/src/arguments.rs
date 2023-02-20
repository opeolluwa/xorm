use clap::*;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
/// parse the arguments and return the sub command
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
    ///create the database in the specified DATABASE_URL    
    pub create: String, //path to create the application
    ///drop the created database
    pub drop: String,
}

/// create the init command, essentially a struct to hold the init command arguments and options
#[derive(Args, Debug)]
pub struct MigrationsCommands {
    /// add a new migration
    pub add: String,
    /// run a migration
    pub run: String,
    ///revert a migration
    pub revert: String,
}
