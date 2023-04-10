extern crate mongodb;
pub mod models;
pub mod types;
pub mod utils;
pub mod db;
pub mod repository;
use mongodb::bson::{doc};
use rocket::State;
use rocket::serde::{json::Json};
use models::select::Select;
use models::option::OptionSelect;
use models::option::OptionSelectReceive;

use db::create_connection;
use db::insert_doc;
use db::get_by_id;
use db::update_push;
use db::get_all;
use repository::map;
use utils::ReturnId;
use utils::StateCustom;
use utils::ReturnError;
use utils::string_to_object_id;

#[macro_use]
extern crate rocket;
#[rocket::main]
async fn main() {
    let client = create_connection().await;
    let _ =rocket::build()
      .manage(StateCustom::new(client))
      .mount("/", routes![welcome])
      .mount("/options/", routes![add_option])
      .mount("/selects/",routes![get_select])
      .launch()
      .await;
}

#[get("/")]
async fn welcome() -> String{
  String::from("Welcome Luck Bustard")
}

#[get("/<id>")]
async fn get_select(id:&str,client:&State<StateCustom>){
  println!("{:?}",get_all::<Select>(&client.client, "crabs_test", "selects", map("select",id)).await);
}

#[post("/add",data="<data>")]
async fn add_option(data:Json<OptionSelectReceive>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnError>>{
  let mut option = data.0.convert();

  // FORCE ID, IF SUPPLIED ,TO NONE
  if let Some(id) = option.get_id(){
    let _ = &option
    .set_id(None)
    .build();
  }


  // VALIDATE SELECT ID 
  if let Some(select_id) = option.get_select_id(){
    let select_requested = get_by_id::<Select>(&client.client,"crabs_test","selects",select_id.to_hex().as_str()).await;
    if select_requested == None{
      return Err(Json(ReturnError::new("Select with the given id doesn't exist".to_string())))
    }
  }

  // ID OF CREATED OPTION
  let option_id = insert_doc(&client.client, "crabs_test", "options", &option).await.unwrap().inserted_id.as_object_id().unwrap().to_string();

  // UPDATE OPTIONS IN THE SELECT
  if let Some(select_id) = option.get_select_id(){
    let document = doc! { "$push": { "options": string_to_object_id(&option_id) } };
    update_push::<Select>(&client.client, "crabs_test", "selects", document, select_id.to_hex().as_str()).await;
  }

  Ok(Json(ReturnId::new(option_id)))
}
