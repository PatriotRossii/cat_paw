use rusqlite::{Connection, Result, params};

#[derive(Debug, Clone)]
pub struct Password {
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

    pub fn load_all(conn: &Connection) -> rusqlite::Result<Vec<Self>> {
        let mut statement = conn.prepare("SELECT id, name, password FROM passwordss")?;
        let mut rows = statement.query([])?;

        let mut result = Vec::new();

        while let Some(row) = rows.next()? {
            result.push(Self {
                id: Some(row.get(0).unwrap()),
                name: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
            });
        }

        Ok(result)
    }
}

pub struct PasswordQuery {
    id: Option<Vec<i32>>,
    name: Option<Vec<String>>,
    password: Option<Vec<String>>,
}

impl Default for PasswordQuery {
    fn default() -> Self {
        Self {
            id: None,
            name: None,
            password: None,
        }
    }
}

impl PasswordQuery {
    pub fn id(&mut self, id: i32) {
        if let Some(e) = &mut self.id {
            e.push(id);
        }
    }

    pub fn name<T>(&mut self, name: T)
    where
        T: ToString
    {
        if let Some(e) = &mut self.name {
            e.push(name.to_string());
        }
    }

    pub fn password<T>(&mut self, password: T)
    where
        T: ToString
    {
        if let Some(e) = &mut self.password {
            e.push(password.to_string());
        }
    }

    pub fn query(&self, conn: &Connection) -> rusqlite::Result<Vec<Password>> {
        let mut query = String::from("SELECT id, name, password FROM passwords");
        let mut passwords: Vec<Password> = Vec::new();
        
        let mut query_parts = vec![];
        if let Some(e) = &self.id {
            query_parts.push(
                format!("id IN ({})", e.iter().map(i32::to_string).collect::<Vec<String>>().join(", "))
            );
        }
        if let Some(e) = &self.name {
            query_parts.push(
                format!("name IN ({})", e.iter().map(|x| format!("'{}'", x)).collect::<Vec<String>>().join(", "))
            );
        }
        if let Some(e) = &self.password {
            query_parts.push(
                format!("password IN ({})", e.iter().map(|x| format!("'{}'", x)).collect::<Vec<String>>().join(", "))
            );
        }

        if self.id.is_some() || self.name.is_some() || self.password.is_some() {
            let query_part = format!("WHERE {}", query_parts.join(" OR "));
            query.push_str(&query_part);
        }

        let mut statement = conn.prepare(&query)?;
        let mut rows = statement.query([])?;

        while let Some(row) = rows.next()? {
            passwords.push(Password {
                id: Some(row.get(0).unwrap()),
                name: row.get(1).unwrap(),
                password: row.get(2).unwrap(),
            });
        }
        
        Ok(passwords)
    }
}