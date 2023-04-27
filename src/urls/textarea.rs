use rocket::{serde::json::Json, State};
use crate::{
  models::{
    textarea::{TextArea}
  },
  utils::{
    StateCustom, ReturnError, ReturnMessage, ReturnErrors, ReturnId
  },
  views::{
    textarea::{
      get_textarea_view,
      add_textarea_view,
      delete_textarea_view,
      update_textareas_view
    }
  }
};

#[get("/<id>")]
pub async fn get_textarea_by_id<'a>(id:&'a str,client:&'a State<StateCustom>) -> Result<Json<TextArea>,Json<ReturnError<'a>>>{
  get_textarea_view(&id, &client.client).await
}


#[post("/add",data="<data>")]
pub async fn add_textarea(data:Json<TextArea>,client:&State<StateCustom>) -> Result<Json<ReturnId>, Json<ReturnError>>{
  add_textarea_view(data, &client.client).await
}

#[put("/<id>",data="<data>")]
pub async fn update_textarea<'a>(id:&'a str,data:Json<TextArea>,client:&'a State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnErrors>>{
  update_textareas_view(id, data.0, &client.client).await
}

#[delete("/<id>")]
pub async fn delete_textarea<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  delete_textarea_view(id, &client.client).await
}