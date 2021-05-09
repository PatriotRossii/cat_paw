use rusqlite::Connection;

use super::Model;

struct PasswordsModel { }

impl Model for PasswordsModel {
    fn create(conn: &Connection) -> rusqlite::Result<usize> {
        conn.execute(
            "CREATE TABLE passwords (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                password TEXT NOT NULL,
            )", []
        )
    }

    fn destroy(conn: &Connection) -> rusqlite::Result<usize> {
        conn.execute(
            "DROP TABLE passwords", []
        )
    }
}