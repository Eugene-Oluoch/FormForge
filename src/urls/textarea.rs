use rocket::{serde::json::Json, State};
use crate::{
  models::{
    textarea::{TextArea}
  },
  utils::{
    StateCustom, ReturnError
  },
  views::{
    textarea::{
      get_textarea_view,
      add_textarea_view
    }
  }
};

#[get("/<id>")]
pub async fn get_textarea_by_id<'a>(id:&'a str,client:&'a State<StateCustom>) -> Result<Json<TextArea>,Json<ReturnError<'a>>>{
  get_textarea_view(&id, &client.client).await
}


#[post("/add",data="<data>")]
pub async fn add_textarea(data:Json<TextArea>,client:&State<StateCustom>){
  add_textarea_view(data.0, &client.client).await;
}

#[put("/<id>",data="<data>")]
pub async fn update_textarea<'a>(id:&'a str,data:Json<TextArea>,client:&'a State<StateCustom>){
}

#[delete("/<id>")]
pub async fn delete_textarea<'a>(id:&str,client:&State<StateCustom>){
}