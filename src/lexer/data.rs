use core::fmt;

#[derive(Debug, Clone)]
pub enum Data {
    Float(f64),
    String(String),
    Boolean(bool),
    None,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Data::Float(n) => write!(f, "{n}"),
            Data::String(s) => write!(f, "{s}"),
            Data::Boolean(b) => write!(f, "{b}"),
            Data::None => write!(f, "nil"),
        }
    }
}
