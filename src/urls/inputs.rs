use rocket::State;
use rocket::serde::json::Json;
use crate::models::{input::Input, traits::ResetDefaults};
use crate::db::{insert_doc};
use crate::utils::StateCustom;

#[post("/add", data="<data>")]
pub async fn add_input(data:Json<Input>,client:&State<StateCustom>){
  let mut input = data.0;
  let _ = &mut input.reset();
  insert_doc(&client.client, "crabs_test", "inputs", &input).await;
}