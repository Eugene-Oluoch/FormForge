use crate::utils::{ReturnError, StateCustom, string_to_object_id, ReturnId};
use crate::db::{get_by_id,insert_doc, update_push};
use crate::OptionSelect;
use crate::Select;
use mongodb::bson::doc;
use rocket::State;
use rocket::serde::json::Json;
use uuid::Uuid;


// NOTE -> MUST CATCH ERROR THROWN BY GET_BY_ID
// CORRECT STATUS CODE HAVEN'T BEEN MAPPED YET
#[get("/<id>")]
pub async fn get_option_by_id(id:&str,client:&State<StateCustom>) -> Result<Json<OptionSelect>,Json<ReturnError>>{
  let option_data = get_by_id::<OptionSelect>(&client.client, "crabs_test", "options", id).await.expect("Failed on db level");
  if let Some(result) = option_data{
    Ok(Json(result))
  }else{
    Err(Json(ReturnError::new("Option with the provided id doesn't exists.".to_string())))
  }
}


#[post("/add",data="<data>")]
pub async fn add_option(data:Json<OptionSelect>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnError>>{
  let mut option = data.0;

  // GENERATE A RANDOM ID FOR OPTION
  let _ = &option
    .set_id(Some(Uuid::new_v4().to_string()))
    .build();

  // VALIDATE SELECT ID 
  if let Some(select_id) = option.get_select_id(){
    let select_requested = get_by_id::<Select>(&client.client,"crabs_test","selects",select_id.as_str()).await.expect("Failed on db level");
    if select_requested == None{
      return Err(Json(ReturnError::new("Select with the given id doesn't exist".to_string())))
    }
  }

  // ID OF CREATED OPTION
  let option_id = insert_doc(&client.client, "crabs_test", "options", &option).await.unwrap().inserted_id.to_string();

  // UPDATE OPTIONS IN THE SELECT
  if let Some(select_id) = option.get_select_id(){
    let document = doc! { "$push": { "options": &option_id } };
    update_push::<Select>(&client.client, "crabs_test", "selects", document, select_id).await;
  }

  Ok(Json(ReturnId::new(option_id.trim_matches('"').to_string())))
}
