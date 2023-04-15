use mongodb::bson::doc;
use rocket::State;
use rocket::serde::json::Json;
use crate::models::{
  input::{Input}, 
  traits::{ResetDefaults},
  form::{Form}
};
use crate::db::{
  insert_doc,
  get_by_id,
  update_push
};
use crate::utils::{
  StateCustom,
  ReturnError,
  ReturnId
};

#[get("/<id>")]
pub async fn get_input<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<Input>,Json<ReturnError<'a>>>{
  let input_data = get_by_id::<Input>(&client.client, "crabs_test", "inputs", id).await.expect("Failed");
  if let Some(result) = input_data{
    Ok(Json(result))
  }else{
    Err(Json(ReturnError::new("Input with the provided id doesn't exists. 🙁")))
  }
}

/*
CASES TO COVER
--------------
Form-ID -> If supplied confirm if it exist , and if it exists, update form by attaching the newly added option

*/

#[post("/add", data="<data>")]
pub async fn add_input(data:Json<Input>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnError>>{
  let mut input = data.0;
  let _ = &mut input.reset();
  let _ = &mut input.map_type();

  // FORM ID 
  if let Some(form) = &input.form_id{
    let form_result = get_by_id::<Form>(&client.client, "crabs_ke", "forms", form).await.expect("Failed");
    if form_result == None {
      return Err(Json(ReturnError::new("Form with the provided id doesn't exists 🙁")))
    }
  }
  let input_id = insert_doc(&client.client, "crabs_test", "inputs", &input).await.unwrap().inserted_id.to_string();

  // UPDATE FORM'S Inputs
  if let Some(form) = &input.form_id{
    let document = doc! { "$push": { "inputs": &input_id.trim_matches('"').to_string() } };
    update_push::<Form>(&client.client, "crabs_test", "forms", document, form).await;
  }

  Ok(Json(ReturnId::new(&input_id)))

}