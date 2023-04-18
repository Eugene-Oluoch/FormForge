use rocket::State;
use rocket::serde::json::Json;
use crate::models::form::{FormReceive};
use crate::utils::{StateCustom, ReturnMessage, ReturnId, ReturnErrors};
use crate::{
  views::{
    forms::{get_form_view,add_form_view,validate}
  }
};

#[get("/<id>")]
pub async fn get_form<'a>(id:String,client:&State<StateCustom>) -> Result<Json<FormReceive>,Json<ReturnMessage<'a>>>{
  get_form_view(id, &client.client).await
}


#[post("/add",data="<data>")]
pub async fn add_form(data:Json<FormReceive>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnErrors>>{
  if let Some(errors) = validate(&data.0).await{
    Err(Json(errors))
  }else{
    Ok(add_form_view(data, &client.client).await)
  }
}