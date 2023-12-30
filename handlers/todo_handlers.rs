use actix_web::{web, HttpResponse};
use diesel::prelude::*;
use crate::db::establish_connection;
use crate::models::todo::NewTodo;
use crate::routes::todo_routes::Item;

pub async fn get_todos() -> HttpResponse {
    let items: Vec<Item> = vec![
        Item { id: 1, name: "Item 1".to_string() },
        Item { id: 2, name: "Item 2".to_string() },
    ];

    // Retorna os itens como JSON
    HttpResponse::Ok().json(items)
}

pub async fn create_todo(new_todo: web::Json<NewTodo>) -> HttpResponse {
    let item: Item = Item { id: 1, name: "Item 1".to_string() };
    HttpResponse::Ok().json(item)
}

pub async fn get_todo(path: web::Path<(i32,)>) -> HttpResponse {
    let item: Item = Item { id: 1, name: "Item 1".to_string() };
    HttpResponse::Ok().json(item)
}

pub async fn update_todo(path: web::Path<(i32,)>, updated_todo: web::Json<NewTodo>) -> HttpResponse {
    let item: Item = Item { id: 1, name: "Item 1".to_string() };
    HttpResponse::Ok().json(item)
}

pub async fn delete_todo(path: web::Path<(i32,)>) -> HttpResponse {
    HttpResponse::Ok().json(true)
}