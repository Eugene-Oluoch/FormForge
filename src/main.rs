extern crate mongodb;
pub mod models;
pub mod validation;
pub mod utils;
pub mod db;
pub mod repository;
pub mod urls;
pub mod views;
use mongodb::bson::{doc};
use utils::{StateCustom, ReturnError};
use db::create_connection;
use models::{
  select::{SelectReceive},
  option::{OptionSelect}
};
use urls::{
  options::{get_option_by_id,add_option,delete_option, update_option},
  selects::{get_select_by_id,add_select,update_select,delete_select},
  forms::{get_form,add_form,update_form,delete_form},
  inputs::{get_input,add_input,update_input,delete_input},
  validate::{validate_data},
  textarea::{get_textarea_by_id,add_textarea,update_textarea,delete_textarea}
};

// NOTE -> You might encounter String types but am planning to convert to &str
// NOTE -> Code has alot of unhandled Result enum Err -> To Be Fixed
#[macro_use]
extern crate rocket;
use rocket::{serde::json::Json};

#[rocket::main]
async fn main() {
    let client = create_connection().await;
    let _ = rocket::build()
      .manage(StateCustom::new(client))
      .register("/", catchers![error_processing_data])
      .mount("/", routes![welcome])
      .mount("/validate",routes![validate_data])
      .mount("/options/", routes![get_option_by_id,add_option,update_option,delete_option])
      .mount("/selects/",routes![get_select_by_id,add_select,update_select,delete_select])
      .mount("/inputs", routes![get_input,add_input,update_input,delete_input])
      .mount("/forms/",routes![get_form,add_form,update_form,delete_form])
      .mount("/textareas/",routes![get_textarea_by_id,add_textarea,update_textarea,delete_textarea])
      .launch()
      .await;
}


#[get("/")]
async fn welcome() -> &'static str{
  "Welcome to FormForge 🙂"
}

#[catch(422)]
fn error_processing_data() -> Json<ReturnError<'static>> {
    Json(ReturnError::new("Data supplied isn't valid, confirm and try again"))
}

// ELIMINATE THE HUSTLE OF PASSING CLIENT FOR ALL DB QUERIES -> CREATE A CONNECTION ONCE FOR ALL THREAD OPERATIONS
// CONVERT TEXTAREA AND SELECT METHOD TO USE ONE REUSABLE FUNCTION