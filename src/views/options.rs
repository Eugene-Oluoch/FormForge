use mongodb::{Client, bson::doc};
use rocket::serde::json::Json;

use crate::{
  models::{
  option::{OptionSelect},
  select::{Select}, 
  traits::{ResetDefaults}
}, 
utils::{
  ReturnId, 
  ReturnError,
  ReturnMessage
},
db::{
  get_by_id,
  insert_doc,
  update_one
}
};

pub async fn get_option_view<'a>(id:&str,client:&Client) -> Result<Json<OptionSelect>,Json<ReturnError<'a>>>{
  let option_data = get_by_id::<OptionSelect>(client,"options",id).await.expect("failed");
  if let Some(result) = option_data{
    if result.archive == Some(true){
      Err(Json(ReturnError::new("Option with the provided id doesn't exists.")))
    }else{
      Ok(Json(result))
    }
  }else{
    Err(Json(ReturnError::new("Option with the provided id doesn't exists.")))
  }
}

pub async fn add_option_helper(option:&mut OptionSelect,client:&Client) -> Result<String,String>{
  // RESET AND ASSIGN ID
  let _ = option.reset();

  // VALIDATE SELECT ID 
  if let Some(select_id) = &option.select_id{
    let select_requested = get_by_id::<Select>(client,"selects",select_id.as_str()).await.expect("Failed on db level");
    if select_requested == None{
      return Err("Select with the given id doesn't exist".to_string())
    }
  }

  // ID OF CREATED OPTION
  let option_id = insert_doc(client, "options", &option).await.unwrap().inserted_id.to_string();

  // UPDATE OPTIONS IN THE SELECT
  if let Some(select_id) = &option.select_id{
    let document = doc! { "$push": { "options": &option_id.trim_matches('"').to_string() } };
    update_one::<Select>(client,"selects", document, select_id).await;
  }

  Ok(option_id.trim_matches('"').to_string())

}

pub async fn add_option_view(data:Json<OptionSelect>,client:&Client) -> Result<Json<ReturnId>,Json<ReturnError>> {
  let mut option = data.0;
  let results = add_option_helper(&mut option, client).await;
  if let Ok(result) = results{
    Ok(Json(ReturnId::new(result.as_str())))
  }else{
    Err(Json(ReturnError::new("Failed")))
  }
}


pub async fn delete_option_view<'a>(id:&str,client:&Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  let update_query = doc! {"$set":{"archive":true}};
  let results = update_one::<OptionSelect>(client, "options",update_query,id).await;
  if let Ok(_) = &results{
    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  }else {
    Err(Json(ReturnError::new("Failed to delete 🙁")))
  }

}