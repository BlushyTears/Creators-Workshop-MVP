/*
    - THIS IS AN MVP -

    Creators Workshop: A Web service built using Actix which lets a creators submit their skills
    and buyers can request the creators build customized products in exchange for money. 

    EXAMPLE: Alice is a carpenter and she made a profile on the site. Bob then makes a request
    from Alice to make a chair for him. If Alice accepts, Bob makes a payment, and then Alice
    will make the chair, and then ship it to him. 

    Name of Project: Creators Workshop
    Name of Domain: N/A

    Technologies (RUST): Actix, handlebars (html), Diesel for database
*/

#[macro_use]
extern crate serde_json;

use actix_web::{web, get, App, HttpResponse, HttpServer};
use std::io;

use handlebars::Handlebars;

#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "name": "Creators Workshop"
    });
    let body = hb.render("index", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
