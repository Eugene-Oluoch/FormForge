use crate::models::traits::ResetDefaults;
use crate::utils::{ReturnError, StateCustom,ReturnId,ReturnMessage};
use crate::db::{get_by_id,insert_doc, update_push,delete_by_id,get_all,update_many,update_one};
use crate::OptionSelect;
use crate::Select;
use mongodb::bson::doc;
use rocket::State;
use rocket::serde::json::Json;
use crate::views::{
  options::{add_option_view}
};


// TODO MAP CORRECT STATUS CODES
#[get("/<id>")]
pub async fn get_option_by_id<'a>(id:&'a str,client:&'a State<StateCustom>) -> Result<Json<OptionSelect>,Json<ReturnError<'a>>>{
  let option_data = get_by_id::<OptionSelect>(&client.client, "crabs_test", "options", id).await.expect("Failed on db level");
  if let Some(result) = option_data{
    Ok(Json(result))
  }else{
    Err(Json(ReturnError::new("Option with the provided id doesn't exists.")))
  }
}

#[post("/add",data="<data>")]
pub async fn add_option(data:Json<OptionSelect>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnError>>{
  let mut option = data.0;

  let results = add_option_view(&mut option, &client.client).await;
  if let Ok(result) = results{
    Ok(Json(ReturnId::new(result.as_str())))
  }else{
    Err(Json(ReturnError::new("Failed")))
  }
}


#[delete("/<id>")]
pub async fn delete_option<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{

  let update = doc! { "$set": {"archive":true} };
  let results = update_one::<OptionSelect>(&client.client, "crabs_test", "options", id, update).await;
  if let Ok(_) = &results{
    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  }else {
    Err(Json(ReturnError::new("Failed to delete 🙁")))
  }

}



// DELETE OPTION FROM ALL SELECTS
// let match_selects = doc! {"options":id};
// let delete =doc! {"$pullAll":{"options":[id]}};
// update_many::<Select>(&client.client, "crabs_test", "selects", match_selects, delete, id).await;
