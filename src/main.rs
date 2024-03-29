extern crate actix_web;
extern crate futures;
extern crate juniper;

use std::env;
use std::io;
use std::sync::Arc;

use actix_web::{web, App, Error, HttpResponse, HttpServer};
use futures::future::Future;
use juniper::http::graphiql::graphiql_source;
use juniper::http::GraphQLRequest;

mod graphql_schema;

use crate::graphql_schema::{create_schema, Schema};

fn main() -> io::Result<()> {
    let schema = std::sync::Arc::new(create_schema());
    let url = get_base_url();
    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .service(web::resource("/graphql").route(web::post().to_async(graphql)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql)))
    })
    .bind(url)?
    .run()
}

fn graphql(
    st: web::Data<Arc<Schema>>,
    data: web::Json<GraphQLRequest>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    web::block(move || {
        let res = data.execute(&st, &());
        Ok::<_, serde_json::error::Error>(serde_json::to_string(&res)?)
    })
    .map_err(Error::from)
    .and_then(|user| {
        Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(user))
    })
}

fn graphiql() -> HttpResponse {
        let html = graphiql_source("http://localhost:3003/graphql");
        HttpResponse::Ok()
            .content_type("text/html; charset=utf-8")
            .body(html)
    }

fn get_env_string<'a>(key: &'a str, default: &'a str) -> String {
    env::var(key).unwrap_or_else(|_error| default.to_owned())
}

fn get_base_url() -> String {
    let host = get_env_string("HOST", "0.0.0.0");
    let port = get_env_string("PORT", "3003");
    host + ":" + port.as_str()
}
