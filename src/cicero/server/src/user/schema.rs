
use opus::db::OpusSqlDb;
use opus::preludes::*;
use opus::Error;

/// Create all necessary tables in SQLite for a new user
pub fn create(db: &OpusSqlDb<Sqlite>) -> Result<(), Error> {
    tasks_tables(db)?;
    workspaces_tables(db)?;
    reminders_tables(db)?;
    Ok(())
}

/// Create tasks-related tables
fn tasks_tables(db: &OpusSqlDb<Sqlite>) -> Result<(), Error> {
    let tasks_sql = r#"
        CREATE TABLE tasks (
            id INTEGER PRIMARY KEY,
            status VARCHAR(30) NOT NULL,
            name TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        )"#;

    let tasks_items_sql = r#"
        CREATE TABLE tasks_items (
            id INTEGER PRIMARY KEY,
            task_id INT NOT NULL,
            status VARCHAR(30) NOT NULL,
            plugin VARCHAR(255) NOT NULL,
            action VARCHAR(255) NOT NULL,
            summary TEXT NOT NULL,
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP,
            FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE CASCADE
        )"#;

    db.execute(tasks_sql, &[])?;
    db.execute(tasks_items_sql, &[])?;
    Ok(())
}

/// Create workspaces tables
fn workspaces_tables(db: &OpusSqlDb<Sqlite>) -> Result<(), Error> {
    let workspaces_sql = r#"
        CREATE TABLE workspaces (
            id INTEGER PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            owner_id INT NOT NULL,  -- Ties to user profile
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP
        )"#;

    db.execute(workspaces_sql, &[])?;
    Ok(())
}

/// Create reminders tables
fn reminders_tables(db: &OpusSqlDb<Sqlite>) -> Result<(), Error> {
    let reminders_sql = r#"
        CREATE TABLE reminders (
            id INTEGER PRIMARY KEY,
            task_id INT,  -- Optional link to tasks
            description TEXT NOT NULL,
            due_at TIMESTAMP NOT NULL,
            status VARCHAR(30) NOT NULL DEFAULT 'pending',
            created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP,
            FOREIGN KEY (task_id) REFERENCES tasks (id) ON DELETE SET NULL
        )"#;

    db.execute(reminders_sql, &[])?;
    Ok(())
}



