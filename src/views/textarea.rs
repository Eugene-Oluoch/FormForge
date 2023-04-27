use mongodb::Client;
use rocket::serde::json::Json;

use crate::{
  db::{
    get_by_id,
    insert_doc
  },
  models::{
    textarea::{TextArea}, traits::ResetDefaults
  },
  utils::{
    ReturnError
  }
};

pub async fn get_textarea_view<'a>(id:&'a str,client:&'a Client) -> Result<Json<TextArea>,Json<ReturnError<'a>>> {
  let results = get_by_id::<TextArea>(client, "textareas", id).await.expect("Failed");
  if let Some(val) = results{
    Ok(Json(val))
  }else{
    Err(Json(ReturnError::new("Textarea with the provided id doesn't exists 🙁")))
  }
}

pub async fn add_textarea_view(mut data:TextArea,client:&Client){
  data.reset();

  // Handle Form id if supplied

  println!("{:?}",insert_doc(client, "textareas", &data).await);

}