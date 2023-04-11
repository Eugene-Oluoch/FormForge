extern crate mongodb;
pub mod models;
pub mod types;
pub mod utils;
pub mod db;
pub mod repository;
pub mod urls;
use mongodb::bson::{doc};
use models::select::{Select};
use models::option::{OptionSelect};
use utils::StateCustom;

// DB IMPORTS
use db::create_connection;

// URL IMPORTS
use urls::options::{get_option_by_id,add_option};
use urls::selects::{get_select_by_id};

// bson::from_bson(bson::Bson::Document(doc)).unwrap() -> To convert mongo Document to struct
// NOTE -> You might encounter String types but am planning to convert to &str

#[macro_use]
extern crate rocket;
#[rocket::main]
async fn main() {
    let client = create_connection().await;
    let _ =rocket::build()
      .manage(StateCustom::new(client))
      .mount("/", routes![welcome])
      .mount("/options/", routes![get_option_by_id,add_option])
      .mount("/selects/",routes![get_select_by_id])
      .mount("/forms/",routes![])
      .launch()
      .await;
}

#[get("/")]
async fn welcome() -> String{
  String::from("Welcome to FormForge 🙂")
}
