extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use actix_web::{http, server, App, Json, Path, Responder};
use std::error::Error;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct DataResponse {
    msg: String,
}

fn index(info: Path<(u32, String)>) -> impl Responder {
    format!("Hello {}!, id: {}", info.1, info.0)
}

fn json(data: Json<Data>) -> Json<DataResponse> {
    Json(DataResponse {
        msg: data.msg.clone(),
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    server::new(|| {
        App::new()
            .route("/{id}/{name}/index.html", http::Method::GET, index)
            .route("/json", http::Method::POST, json)
    }).bind("127.0.0.1:8080")?
        .run();

    Ok(())
}
