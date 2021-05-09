use rusqlite::Connection;

pub mod passwords;

pub trait Model {
    fn create(conn: &Connection) -> rusqlite::Result<usize>;
    fn destroy(conn: &Connection) -> rusqlite::Result<usize>;
}