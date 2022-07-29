use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum AstNode {
    KeyValue {
        key: String,
        value: Box<AstNode>,
    },
    KeyValueList {
        value: Vec<AstNode>,
        map: HashMap<String, AstNode>,
    },
    String {
        value: String,
    },
    Integer {
        value: i64,
    },
    Float {
        value: f64,
    },
    List {
        value: Vec<AstNode>,
    },
    Default,
}

impl AstNode {
    pub fn to_kv(&self) -> (String, Self) {
        match self.clone() {
            Self::KeyValue { key, value } => (key, *value),
            _ => unreachable!(),
        }
    }
}

impl Default for AstNode {
    fn default() -> Self {
        Self::Default
    }
}