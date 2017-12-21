use parser::ASTNode;

use std::fmt;

#[derive(PartialEq, Clone)]
pub enum Value {
    Symbol(String),
    Integer(isize),
    Boolean(bool),
    StringValue(String),
    List(Vec<Value>),
    Func(Vec<String>, Vec<ASTNode>),
}

use self::Value::*;

#[macro_export]
macro_rules! empty { () => (List(vec![])) }

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl Value {
    fn to_string(&self) -> String {
        match self {
            &Symbol(_) => format!("'{}", self.display()),
            &List(_) => format!("'{}", self.display()),
            _ => self.display()
        }
    }

    fn display(&self) -> String {
        match self {
            &Symbol(ref val) => format!("{}", val),
            &Integer(val) => format!("{}", val),
            &Boolean(val) => format!("#{}", if val { "t" } else { "f" }),
            &StringValue(ref val) => format!("\"{}\"", val),
            &List(ref val) => {
                let mut s = String::new();
                let mut first = true;
                for n in val.iter() {
                    if first {
                        first = false;
                    } else {
                        s.push_str(" ");
                    }
                    s.push_str(&n.display());
                }
                format!("({})", s)
            }
            &Func(_, _) => format!("#<procedure>")
        }
    }
}
