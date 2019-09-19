extern crate actix_web;

use std::env;
use std::io;
use std::string;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

fn main() -> io::Result<()> {

    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
    })
    .bind("localhost:3003")?
    .run()
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello World")
}
