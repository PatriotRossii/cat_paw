use rusqlite::Connection;

pub mod passwords;

pub enum FieldAttribute {
    Integer,
    Text,
    NotNull,
    PrimaryKey,
    Autoincrement,
}

impl FieldAttribute {
    pub const fn as_str(&self) -> &'static str {
        match self {
            FieldAttribute::Integer => { "INTEGER "}
            FieldAttribute::Text => { "TEXT "}
            FieldAttribute::NotNull => { "NOT NULL "}
            FieldAttribute::PrimaryKey => { "PRIMARY KEY" }
            FieldAttribute::Autoincrement => { "AUTOINCREMENT "}
        }
    }
}

pub struct Field {
    name: &'static str,
    attributes: &'static [FieldAttribute]
}

impl Field {
    pub const fn new(name: &'static str, attributes: &'static [FieldAttribute]) -> Self {
        Self {
            name,
            attributes
        }
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        format!("{} {}", self.name, self.attributes.iter().map(|x| x.as_str().to_string()).collect::<Vec<String>>().join(" "))
    }
}

pub trait Model<const N: usize> {
    const TABLE_NAME: &'static str;
    const TABLE_COLUMNS: [Field; N];
    
    fn create(conn: &Connection) -> rusqlite::Result<usize> {
        let query = format!("CREATE TABLE({} {})", Self::TABLE_NAME, Self::TABLE_COLUMNS.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",\n"));
        conn.execute(&query, [])
    }

    fn destroy(conn: &Connection) -> rusqlite::Result<usize> {
        let query = format!("DROP TABLE {}", Self::TABLE_NAME);
        conn.execute(&query, [])
    }
}