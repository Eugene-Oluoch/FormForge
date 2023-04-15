use mongodb::bson::{doc};
use rocket::State;
use rocket::serde::json::Json;
use crate::utils::{StateCustom, ReturnError, ReturnId,ReturnMessage};
use crate::{SelectReceive};
use crate::views::{
  selects::{get_select_view,add_select_view,delete_select_view}
};

// TODO MAP CORRECT STATUS CODES
// TODO IGNORE ALL OPTIONS WITH FIELD ARCHIVE SET TO TRUE
#[get("/<id>")]
pub async fn get_select_by_id(id:String,client:&State<StateCustom>) -> Result<Json<SelectReceive>,Json<ReturnError>>{
  get_select_view(id, &client.client).await
}


#[post("/add",data="<data>")]
pub async fn add_select(data:Json<SelectReceive>,client:&State<StateCustom>) -> Result<Json<ReturnId>, Json<ReturnError>>{
  add_select_view(data, &client.client).await
}


#[delete("/<id>")]
pub async fn delete_select<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  delete_select_view(id, &client.client).await
}