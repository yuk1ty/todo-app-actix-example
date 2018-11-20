use schema::todos;

#[derive(Debug, Queryable, PartialEq, Identifiable, Serialize, Deserialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub body: Option<String>,
    pub status: bool,
}

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String, // ここでコピーが走るのがちょっとアホらしい
    pub body: Option<String>,
    pub status: bool,
}
