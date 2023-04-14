use rocket::State;
use rocket::serde::json::Json;
use uuid::Uuid;
use crate::models::form::Form;
use crate::db::{insert_doc};
use crate::utils::StateCustom;


#[post("/add",data="<data>")]
pub async fn add_form(data:Json<Form>,client:&State<StateCustom>){
  let mut form = data.0;

  // GENERATE A RANDOM ID FOR FORM
  let _ = form._id=Some(Uuid::new_v4().to_string());

  // ID OF CREATED FORM
  let option_id = insert_doc(&client.client, "crabs_test", "forms", &form).await.unwrap().inserted_id.to_string();

}

