use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use serde::Deserialize;

#[get("/")]
async fn root() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}

#[derive(Deserialize)]
struct Book {
    title: String,
    author: String,
    date_published: u16,
    isbn: String,
}

#[post("/books")]
async fn books(book: web::Json<Book>) -> Result<String> {
    Ok(format!(
        "Title: {}\n\
        Author: {}\n\
        Published: {}\n\
        isbn: {}",
        book.title, book.author, book.date_published, book.isbn
    ))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(root).service(books))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
