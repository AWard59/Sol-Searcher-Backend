#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] 
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
struct User {
  id: u32,
  name: String,
  age: u8,
  // email: String,
  // location: String
}

// Define some sample data for the API
static mut USERS: Vec<User> = Vec::new();

// Define an endpoint to get all users
#[get("/api/users")]
fn get_all_users() -> Json<Vec<User>> {
    unsafe {
        Json(USERS.clone())
    }
}

// Define an endpoint to get a specific user by ID
#[get("/api/users/<id>")]
fn get_user_by_id(id: u32) -> Json<Option<User>> {
    unsafe {
        let user = USERS.iter().find(|u| u.id == id).cloned();
        Json(user)
    }
}

// Define an endpoint to create a new user
#[post("/api/users", format = "json", data = "<user>")]
fn create_user(user: Json<User>) -> Json<User> {
    unsafe {
        let new_user = user.into_inner();
        USERS.push(new_user.clone());
        Json(new_user)
    }
}

// Define an endpoint to update an existing user
#[put("/api/users/<id>", format = "json", data = "<user>")]
fn update_user(id: u32, user: Json<User>) -> Json<Option<User>> {
    unsafe {
        if let Some(existing_user) = USERS.iter_mut().find(|u| u.id == id) {
            *existing_user = user.into_inner();
            Json(Some(existing_user.clone()))
        } else {
            Json(None)
        }
    }
}

// Define an endpoint to delete a user
#[delete("/api/users/<id>")]
fn delete_user(id: u32) -> Json<Option<User>> {
    unsafe {
        let index = USERS.iter().position(|u| u.id == id);
        if let Some(i) = index {
            Json(Some(USERS.remove(i)))
        } else {
            Json(None)
        }
    }
}

fn main() {
    // Initialize some sample data for the API
    unsafe {
        USERS.push(User { id: 1, name: "Alice".to_string(), age: 30 });
        USERS.push(User { id: 2, name: "Bob".to_string(), age: 25 });
        USERS.push(User { id: 3, name: "Charlie".to_string(), age: 40 });
    }

    // Launch the Rocket web framework
    rocket::ignite()
        .mount("/", routes![get_all_users, get_user_by_id, create_user, update_user, delete_user])
        .launch();
}