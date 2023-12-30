use actix_web::{web, Responder};

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
    // Implementação para obter itens
}

async fn add_item(item: web::Json<Item>) -> impl Responder {
    // Implementação para adicionar um item
}

async fn delete_item(path: web::Path<usize>) -> impl Responder {
    // Implementação para excluir um item
}