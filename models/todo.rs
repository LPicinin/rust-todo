use diesel::{Queryable, Insertable};

#[derive(Debug, Queryable, Identifiable, Insertable)]
#[table_name = "todos"]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug, Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
    pub completed: bool,
}