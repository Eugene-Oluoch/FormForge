use std::result;

use mongodb::bson::{self, from_bson};
use rocket::State;
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::models::form::{Form, FormReceive};
use crate::db::{insert_doc, get_all, get_by_id};
use crate::models::traits::ResetDefaults;
use crate::utils::{StateCustom, ReturnMessage, ReturnId,trim_quotes};
use crate::repository::{map};

// Works but not for forms which have no inputs and selects -> TO BE FIXED
#[get("/<id>")]
pub async fn get_form<'a>(id:String,client:&State<StateCustom>) -> Result<Json<FormReceive>,Json<ReturnMessage<'a>>>{
  let results = get_all::<Form>(&client.client, "crabs_test", "forms", map("form",id.as_str())).await;
  if let Ok(result) = results{
    Ok(Json(from_bson(bson::Bson::Document(result)).expect("Failed here")))
  } else {
    Err(Json(ReturnMessage::new("Failed to get the form 🙁")))
  }
}
#[post("/add",data="<data>")]
pub async fn add_form(data:Json<Form>,client:&State<StateCustom>) -> Json<ReturnId>{
  let mut form = data.0;

  // GENERATE A RANDOM ID FOR FORM
  let _ = &mut form.reset();

  // ID OF CREATED FORM
  let form_id = insert_doc(&client.client, "crabs_test", "forms", &form).await.unwrap().inserted_id.to_string();
  Json(ReturnId::new(&trim_quotes(&form_id.to_string())))
}

