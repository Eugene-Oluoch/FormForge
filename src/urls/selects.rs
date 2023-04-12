use mongodb::bson::{self, from_bson};
use mongodb::error::Error;
use rocket::State;
use rocket::serde::json::Json;
use crate::utils::StateCustom;
use crate::db::{get_all,insert_doc};
use crate::{Select,SelectReceive};
use crate::repository::map;
use uuid::Uuid;

// IGNORE EXPECTS -> TO BE HANDLES LATER

#[get("/<id>")]
pub async fn get_select_by_id(id:String,client:&State<StateCustom>) -> Result<Json<SelectReceive>,String>{
  let document = get_all::<Select>(&client.client, "crabs_test", "selects", map("select",id.as_str())).await;
  if let Ok(doc) = document{
    Ok(Json(from_bson(bson::Bson::Document(doc)).expect("Failed here")))
  } else {
    Err("Select with the given id doesn't exist 🙁".to_string())
  }
}

/* 
Handle case where form_id is supplied:
 -> Confirm if form exists
 -> Update form when select is created
*/
#[post("/add",data="<data>")]
pub async fn add_select(data:Json<SelectReceive>,client:&State<StateCustom>) -> Result<&str, &str>{
  let mut select = data.0;
  
  // GENERATE AN ID FOR THE SELECT FIELD
  let _ = & mut select.set_id(Uuid::new_v4().to_string());


  let results = insert_doc(&client.client, "crabs_test", "selects", &select).await;

  match &results {
    Ok(_) => Ok("Select Added successfully 🙂"),
    Err(_) => Err("Failed to create the select field 🙁")
  }
}