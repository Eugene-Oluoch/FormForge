extern crate mongodb;
pub mod models;
pub mod validation;
pub mod utils;
pub mod db;
pub mod repository;
pub mod urls;
pub mod views;
use mongodb::bson::{doc};
use utils::StateCustom;
use db::create_connection;
use models::{
  select::{SelectReceive},
  option::{OptionSelect}
};
use urls::{
  options::{get_option_by_id,add_option,delete_option},
  selects::{get_select_by_id,add_select,delete_select},
  forms::{get_form,add_form},
  inputs::{get_input,add_input,delete_input}
};

// bson::from_bson(bson::Bson::Document(doc)).unwrap() -> To convert mongo Document to struct
// NOTE -> You might encounter String types but am planning to convert to &str

#[macro_use]
extern crate rocket;
#[rocket::main]
async fn main() {
    let client = create_connection().await;
    let _ = rocket::build()
      .manage(StateCustom::new(client))
      .mount("/", routes![welcome])
      .mount("/options/", routes![get_option_by_id,add_option,delete_option])
      .mount("/selects/",routes![get_select_by_id,add_select,delete_select])
      .mount("/inputs", routes![get_input,add_input,delete_input])
      .mount("/forms/",routes![get_form,add_form])
      .launch()
      .await;
}

#[get("/")]
async fn welcome() -> &'static str{
  "Welcome to FormForge 🙂"
}
