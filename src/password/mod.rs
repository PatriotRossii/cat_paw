use rusqlite::{Connection, Result, params};

#[derive(Debug, Clone)]
struct Password {
    id: Option<i32>,
    name: String,
    password: String,
}

impl Password {
    pub fn new<T1, T2>(name: T1, password: T2) -> Password
    where
        T1: ToString,
        T2: ToString
    {
        Password {
            id: None,
            name: name.to_string(),
            password: password.to_string()
        }
    }

    pub fn save(&self, conn: &Connection) -> Result<usize> {
        conn.execute(
            "INSERT INTO passwords (name, password) VALUES (?1, ?2)",
            params![self.name, self.password]
        )
    }

    pub fn load_all(conn: &Connection) -> Vec<Self> {
        let mut statement = conn.prepare("SELECT id, name, password FROM passwordss").unwrap();
        let mut rows = statement.query([]).unwrap();

        let mut result = Vec::new();

        while let Some(row) = rows.next().unwrap() {
            result.push(Self {
                id: Some(row.get(0).unwrap()),
                name: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
            });
        }

        result
    }
}