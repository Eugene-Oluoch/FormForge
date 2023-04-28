use rocket::State;
use rocket::serde::json::Json;
use crate::models::form::{FormReceive,FormReceiveFinal};
use crate::utils::{StateCustom, ReturnMessage, ReturnId, ReturnErrors, ReturnError};
use crate::{
  views::{
    forms::{get_form_view,add_form_view,update_form_view,validate,delete_form_view}
  }
};

#[get("/<id>")]
pub async fn get_form<'a>(id:String,client:&State<StateCustom>) -> Result<Json<FormReceiveFinal>,Json<ReturnErrors>>{
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

#[put("/<id>",data="<data>")]
pub async fn update_form<'a>(id:&'a str,data:Json<FormReceive>,client:&'a State<StateCustom>)-> Result<Json<ReturnMessage<'a>>,Json<ReturnErrors>>{
  if let Some(errors) = validate(&data.0).await{
    Err(Json(errors))
  }else{
    update_form_view(id, data.0, &client.client).await
  }
}

#[delete("/<id>")]
pub async fn delete_form<'a>(id:&'a str,client:&'a State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  delete_form_view(id,&client.client).await
}
