
use opus::db::OpusSqlDb;
use super::UserSqlDb;

/// Create all necessary tables within SQLite database for new user
pub fn create(db: &UserSqlDb) -> Result<(), OmnidataError> {

    // Tasks tables
    tasks_tables(&db)?;

    Ok(())
}

/// Tasks tables
fn tasks_tables(db: &UserSqlDb) -> Result<(), OmnidataError> {

    let tasks_sql = r#"CREATE TABLE tasks (
        id INTEGER PRIMARY KEY,
        status VARCHAR(30) NOT NULL,
        name TEXT NOT NUL,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP
    )"#;

    let tasks_items_sql = r#"CREATE TABLE tasks_items (
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

    // Execute SQL
    db.execute(&tasks_sql, &[])?;
    db.execute(&tasks_items_sql, &[])?;
    Ok(())
}



