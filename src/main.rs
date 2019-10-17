#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate diesel;
extern crate r2d2_diesel;

use rocket_contrib::json::Json;
use serde_json::{json, Value};

pub mod db;
mod hero;
pub mod schema;
use hero::Hero;

#[post("/", data = "<hero>")]
fn create(hero: Json<Hero>, connection: db::Connection) -> Json<Hero> {
    let insert = Hero {
       // id: id,
        ..hero.into_inner()
    };
    Json(Hero::create(insert, &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
    Json(json!(Hero::read(&connection)))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<Value> {
    let update = Hero {
        //id: id,
        ..hero.into_inner()
    };
    Json(json!({ "success": Hero::update(id, update, &connection) }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
    Json(json!({ "success": Hero::delete(id, &connection) }))
}

fn main() {
    rocket::ignite()
        .mount("/hero", routes![create, update, read])
        .mount("/heroes", routes![read])
        .manage(db::connect())
        .launch();
}
