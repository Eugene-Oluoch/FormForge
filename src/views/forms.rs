use mongodb::{Client, bson::{from_bson, self}};
use rocket::serde::json::Json;

use crate::{
  db::{
    get_all,
    insert_doc
  },
  models::{
    form::{Form,FormReceive}, traits::ResetDefaults
  },
  utils::{
    ReturnMessage,
    ReturnId,
    trim_quotes
  },
  repository::{
    map
  }
};




pub async fn get_form_view<'a>(id:String, client:&Client) -> Result<Json<FormReceive>,Json<ReturnMessage<'a>>>{
  let results = get_all::<Form>(client,"forms", map("form",id.as_str())).await;
  if let Ok(result) = results{
    let mut final_result:FormReceive = from_bson(bson::Bson::Document(result)).expect("failed");

    // RESET SELECTS IF MONGO RETURN A NONE RECORD
    if final_result.selects.len() == 1 && final_result.selects[0]._id == None{
      final_result.selects = Vec::new();
    }

    Ok(Json(final_result))
  } else {
    Err(Json(ReturnMessage::new("Failed to get the form 🙁")))
  }
}


pub async fn add_form_view(data:Json<Form>,client:&Client) -> Json<ReturnId>{
  let mut form = data.0;

  // GENERATE A RANDOM ID FOR FORM
  let _ = &mut form.reset();

  // ID OF CREATED FORM
  let form_id = insert_doc(client, "forms", &form).await.unwrap().inserted_id.to_string();

  Json(ReturnId::new(&trim_quotes(&form_id.to_string())))
}