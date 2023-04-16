use rocket::State;
use rocket::serde::json::Json;
use crate::models::form::{Form, FormReceive};
use crate::utils::{StateCustom, ReturnMessage, ReturnId};
use crate::{
  views::{
    forms::{get_form_view,add_form_view}
  }
};

#[get("/<id>")]
pub async fn get_form<'a>(id:String,client:&State<StateCustom>) -> Result<Json<FormReceive>,Json<ReturnMessage<'a>>>{
  get_form_view(id, &client.client).await
}


#[post("/add",data="<data>")]
pub async fn add_form(data:Json<Form>,client:&State<StateCustom>) -> Json<ReturnId>{
  add_form_view(data, &client.client).await
}


