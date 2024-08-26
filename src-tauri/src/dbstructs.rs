pub struct Set {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_date: String
}

pub struct NewSet<'a> {
    pub name: &'a str,
    pub description: &'a str
}
