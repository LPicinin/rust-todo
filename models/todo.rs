#[macro_use]
extern crate diesel;

use diesel::{Queryable, Insertable, Identifiable};

#[derive(Debug)]
#[table_name = "todos"] // This attribute should work now
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Debug)]
#[table_name = "todos"]
pub struct NewTodo {
    pub title: String,
    pub completed: bool,
}