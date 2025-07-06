use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Path {
    tokens: Vec<String>,
}

impl Display for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.tokens
                .iter()
                .map(|t| t.replace('.', "\\."))
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "path: {:?}", self.tokens)
    }
}

impl From<&str> for Path {
    fn from(value: &str) -> Self {
        let mut token = String::new();
        let mut tokens: Vec<String> = Vec::new();
        let mut chars = value.chars();
        while let Some(ch) = chars.next() {
            match ch {
                '.' => {
                    tokens.push(token);
                    token = String::new();
                }
                '\\' => {
                    if let Some(next) = chars.next() {
                        token.push(next);
                    }
                }
                _ => token.push(ch),
            }
        }
        tokens.push(token);

        Self { tokens }
    }
}

impl Path {
    pub fn new<T: Into<String>>(tokens: Vec<T>) -> Self {
        Self {
            tokens: tokens.into_iter().map(|i| i.into()).collect(),
        }
    }
}

#[derive(Debug)]
struct Value {
    id: u32,
}

#[derive(Debug)]
struct Attribute {
    path: Path,
    id: u32,
}

impl Attribute {
    pub fn get(&self) -> Value {
        // Very expensive operation
        println!("Expensive operation for attribute {}", self.id);
        Value { id: self.id }
    }
}

type Cache = std::collections::HashMap<Path, Value>;

fn main() {
    let attributes = vec![
        Attribute {
            id: 1,
            path: "a0.b0.c0".into(),
        },
        Attribute {
            id: 2,
            path: "a1.b1.c1".into(),
        },
        Attribute {
            id: 3,
            path: "a0.b0.c0".into(),
        },
        Attribute {
            id: 4,
            path: "a1.b1.c0".into(),
        },
        Attribute {
            id: 5,
            path: "a1.b1.c1".into(),
        },
    ];

    let mut cache = Cache::new();

    for attr in attributes {
        let value = cache.entry(attr.path.clone()).or_insert_with(|| attr.get());
        println!("Value for attribute {attr:?}: {value:?}");
    }
}
