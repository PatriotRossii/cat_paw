pub enum Value {
    Integer(i32),
    Text(String)
}

impl Into<Value> for String {
    fn into(self) -> Value {
        Value::Text(self)
    }
}

impl Into<Value> for i32 {
    fn into(self) -> Value {
        Value::Integer(self)
    }
}

impl Into<Value> for &str {
    fn into(self) -> Value {
        Value::Text(self.to_string())
    }
}

impl ToString for Value {
    fn to_string(&self) -> String {
        match self {
            Self::Integer(i) => i.to_string(),
            Self::Text(e) => e.clone()
        }
    }
}

pub enum ClauseRelation {
    EQUAL, IN, OR, AND
}

impl ToString for ClauseRelation {
    fn to_string(&self) -> String {
        match self {
            Self::EQUAL => String::from("="),
            Self::IN => String::from("IN"),
            Self::OR => String::from("OR"),
            Self::AND => String::from("AND"),
        }
    }
}

pub enum ClauseValue {
    Value(Value),
    Clause(Box<Clause>)
}

impl ToString for ClauseValue {
    fn to_string(&self) -> String {
        match self {
            Self::Value(val) => val.to_string(),
            Self::Clause(box clause) => format!("({})", clause.to_string()),
        }
    }
}

impl Into<ClauseValue> for Clause {
    fn into(self) -> ClauseValue {
        ClauseValue::Clause(box self)
    }
}

impl<T> From<T> for ClauseValue
where
    T: Into<Value>
{
    fn from(value: T) -> Self {
        Self::Value(value.into())
    }    
}

pub struct Clause {
    lhs: ClauseValue,
    relation: ClauseRelation,
    rhs: ClauseValue
}

impl Clause {
    fn new<T1, T2>(lhs: T1, relation: ClauseRelation, rhs: T2) -> Self
    where
        T1: Into<ClauseValue>,
        T2: Into<ClauseValue>,
    {
        Self {
            lhs: lhs.into(),
            relation,
            rhs: rhs.into(),
        }
    }
}

impl ToString for Clause {
    fn to_string(&self) -> String {
        format!("{} {} {}", self.lhs.to_string(), self.relation.to_string(), self.rhs.to_string())
    }
}

pub fn main() {
    let clause = Clause::new(Clause::new("id", ClauseRelation::EQUAL, 5), ClauseRelation::OR, Clause::new("id", ClauseRelation::EQUAL, 6));
    println!("{}", clause.to_string())
}