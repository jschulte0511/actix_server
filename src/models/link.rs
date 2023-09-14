use serde::{Deserialize, Serialize};

#[derive(Serialize, Default, Clone, PartialEq)]
pub struct FormLink {
    pub id: Option<i64>,
    pub title: String,
    pub description: String,
    pub url: String,
    pub author: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Link {
    pub id: i32,
    pub category: String,
    pub title: String,
    pub description: String,
    pub url: String,
    pub author: String,
    pub date: String,
    pub active: bool,
}
