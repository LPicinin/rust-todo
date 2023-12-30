use actix_web::{web, Responder, HttpResponse};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    pub id: usize,
    pub name: String,
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/items").route(web::get().to(get_items)));
    cfg.service(web::resource("/items").route(web::post().to(add_item)));
    cfg.service(web::resource("/items/{id}").route(web::delete().to(delete_item)));
}

async fn get_items() -> impl Responder {
    // Simulando a obtenção de itens
    let items: Vec<Item> = vec![
        Item { id: 1, name: "Item 1".to_string() },
        Item { id: 2, name: "Item 2".to_string() },
    ];

    // Retorna os itens como JSON
    HttpResponse::Ok().json(items)
}

async fn add_item(_item: web::Json<Item>) -> impl Responder {
    let item: Item = Item { id: 1, name: "Item 1".to_string() };
    HttpResponse::Ok().json(item)
}

async fn delete_item(_path: web::Path<usize>) -> impl Responder {
    HttpResponse::Ok().json(true)
}