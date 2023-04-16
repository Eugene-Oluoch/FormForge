use mongodb::bson::doc;
use rocket::State;
use rocket::serde::json::Json;
use crate::{
  views::{
    inputs::{add_input_view,get_input_view}
  },
  utils::{
    StateCustom,
    ReturnError,
    ReturnId
  },
  models::{
    input::{Input}
  }
};

#[get("/<id>")]
pub async fn get_input<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<Input>,Json<ReturnError<'a>>>{
  get_input_view(id, &client.client).await
}



#[post("/add", data="<data>")]
pub async fn add_input(data:Json<Input>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnError>>{
  add_input_view(data, &client.client).await
}