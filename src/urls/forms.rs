use mongodb::bson::{self, from_bson};
use rocket::State;
use rocket::serde::json::Json;
use crate::models::form::{Form, FormReceive};
use crate::db::{insert_doc, get_all};
use crate::models::traits::ResetDefaults;
use crate::utils::{StateCustom, ReturnMessage, ReturnId,trim_quotes};
use crate::repository::{map};

#[get("/<id>")]
pub async fn get_form<'a>(id:String,client:&State<StateCustom>) -> Result<Json<FormReceive>,Json<ReturnMessage<'a>>>{
  let results = get_all::<Form>(&client.client, "crabs_test", "forms", map("form",id.as_str())).await;
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
#[post("/add",data="<data>")]
pub async fn add_form(data:Json<Form>,client:&State<StateCustom>) -> Json<ReturnId>{
  let mut form = data.0;

  // GENERATE A RANDOM ID FOR FORM
  let _ = &mut form.reset();

  // ID OF CREATED FORM
  let form_id = insert_doc(&client.client, "crabs_test", "forms", &form).await.unwrap().inserted_id.to_string();
  Json(ReturnId::new(&trim_quotes(&form_id.to_string())))
}

