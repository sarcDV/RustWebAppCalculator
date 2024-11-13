#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Serialize};

#[derive(Serialize)]
struct Calculation {
    result: f64,
}

#[get("/add/<a>/<b>")]
fn add(a: f64, b: f64) -> Json<Calculation> {
    Json(Calculation { result: a + b })
}

#[get("/subtract/<a>/<b>")]
fn subtract(a: f64, b: f64) -> Json<Calculation> {
    Json(Calculation { result: a - b })
}

#[get("/multiply/<a>/<b>")]
fn multiply(a: f64, b: f64) -> Json<Calculation> {
    Json(Calculation { result: a * b })
}

#[get("/divide/<a>/<b>")]
fn divide(a: f64, b: f64) -> Json<Calculation> {
    Json(Calculation { result: a / b })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![add, subtract, multiply, divide])
        .mount("/", rocket::fs::FileServer::from("static"))
}
