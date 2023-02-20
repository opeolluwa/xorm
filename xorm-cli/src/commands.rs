/// create database
pub fn create_database() {
    println!("created database");
}

/// drop database
pub fn drop_database() {
    println!("database dropped");
}

/// add new migration
pub fn add_migrations(name: String) {
    println!("migrations added {name}")
}

/// run migrations
pub fn run_migrations() {
    println!("migrations executed");
}

/// revert migrations
pub fn revert_migrations() {
    println!("reverted migrations");
}
