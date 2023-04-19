#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
  id: u32,
  name: String,
  email: String,
  age: u8,
  location: String
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
  let users = vec![
    User { id: 1, name: String::from("Alice"), email: String::from("alice@example.com"), age: 25, location: String::from("Not here")},
    User { id: 2, name: String::from("Bob"), email: String::from("bob@example.com"), age: 26, location: String::from("Also not here")}
  ];
  Json(users)
}

#[post("/users", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> String {
  format!("User created: {:?}", user)
}

fn main() {
  rocket::ignite()
    .mount("/", routes![get_users, create_user])
    .launch();
}