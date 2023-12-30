use actix_web::{web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::handlers::todo_handlers::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub name: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/items").route(web::get().to(get_todos)));
    cfg.service(web::resource("/items").route(web::post().to(create_todo)));
    cfg.service(web::resource("/items/{id}").route(web::put().to(update_todo)));
    cfg.service(web::resource("/items/{id}").route(web::delete().to(delete_todo)));
    cfg.service(web::resource("/items/{id}").route(web::get().to(get_todo)));
}