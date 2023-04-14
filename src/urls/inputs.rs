use rocket::State;
use rocket::serde::json::Json;
use crate::models::{input::Input, traits::ResetDefaults};
use crate::db::{insert_doc,get_by_id};
use crate::utils::StateCustom;
use crate::utils::ReturnError;

#[get("/<id>")]
pub async fn get_input<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<Input>,Json<ReturnError<'a>>>{
  let input_data = get_by_id::<Input>(&client.client, "crabs_test", "inputs", id).await.expect("Failed");
  if let Some(result) = input_data{
    Ok(Json(result))
  }else{
    Err(Json(ReturnError::new("Input with the provided id doesn't exists. 🙁")))
  }
}

#[post("/add", data="<data>")]
pub async fn add_input(data:Json<Input>,client:&State<StateCustom>){
  let mut input = data.0;
  let _ = &mut input.reset();
  let _ = &mut input.map_type();
  let _ = insert_doc(&client.client, "crabs_test", "inputs", &input).await;
}