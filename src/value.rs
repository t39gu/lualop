use crate::vm::ExeState;
use std::fmt;

#[derive(Clone)]
pub enum Value {
    Nil,
    String(String),
    Function(fn(&ExeState) -> i32),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Nil => write!(f, "Nil"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(_) => write!(f, "function"),
        }
    }
}
