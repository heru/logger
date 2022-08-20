
use actix_web::{HttpResponse, Responder, get};


#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().json("hello from mongodb")
}