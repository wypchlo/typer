use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Set {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_date: String
}

#[derive(Serialize, Deserialize)]
pub struct NewSet<'a> {
    pub name: &'a str,
    pub description: &'a str
}
