use std::result;

use crate::utils::ReturnError;
use crate::utils::StateCustom;
use crate::db::get_by_id;
use crate::OptionSelect;
use crate::OptionSelectReceive;
use rocket::State;
use rocket::serde::json::Json;


// NOTE -> MUST CATCH ERROR THROWN BY GET_BY_ID
#[get("/<id>")]
pub async fn get_option_by_id(id:&str,client:&State<StateCustom>) -> Result<Json<OptionSelectReceive>,Json<ReturnError>>{
  let option_data = get_by_id::<OptionSelect>(&client.client, "crabs_test", "options", id).await.expect("Failed on db level");
  if let Some(result) = option_data{
    Ok(Json(result.convert()))
  }else{
    Err(Json(ReturnError::new("Option with the provided id doesn't exists.".to_string())))
  }
}