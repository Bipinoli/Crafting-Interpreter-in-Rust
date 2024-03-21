use std::fmt;

#[derive(Debug, Clone)]
pub enum Value {
    Num(f64),
    Bool(bool),
    Str(String),
    Nil,
}
impl Value {
    pub fn is_num(&self) -> bool {
        match self {
            Value::Num(_) => true,
            _ => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match self {
            Value::Str(_) => true,
            _ => false,
        }
    }
    pub fn is_bool(&self) -> bool {
        match self {
            Value::Bool(_) => true,
            _ => false,
        }
    }
    pub fn get_num(&self) -> f64 {
        match self {
            Value::Num(v) => v.clone(),
            _ => panic!("can't extract number from non number Value"),
        }
    }
    pub fn get_string(&self) -> String {
        match self {
            Value::Str(s) => s.clone(),
            _ => panic!("can't extract string from non string Value"),
        }
    }
    pub fn get_bool(&self) -> bool {
        match self {
            Value::Bool(b) => b.clone(),
            _ => panic!("can't extract boolean from non boolean Value)"),
        }
    }
}
impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::Bool(v) => write!(f, "{v}"),
            Value::Num(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "{s}"),
        }
    }
}
