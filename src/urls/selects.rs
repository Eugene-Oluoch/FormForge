use mongodb::bson::{self, from_bson, doc};
use mongodb::error::Error;
use rocket::State;
use rocket::serde::json::Json;
use crate::utils::StateCustom;
use crate::db::{get_all,insert_doc, get_by_id, update_push};
use crate::{Select,SelectReceive};
use crate::repository::map;
use crate::Form;
use uuid::Uuid;
use crate::utils::{trim_quotes};

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
TO BE IMPROVED -> TESTING
*/
#[post("/add",data="<data>")]
pub async fn add_select(data:Json<SelectReceive>,client:&State<StateCustom>) -> Result<&str, &str>{
  let mut select = data.0;
  
  // GENERATE AN ID FOR THE SELECT FIELD
  let _ = select._id = Some(Uuid::new_v4().to_string());
  
  // VALIDATION FOR FORM ID
  if let Some(form_id) = &select.form_id{
    let form = get_by_id::<Form>(&client.client, "crabs_test", "forms", form_id.as_str()).await;
    if let Ok(result) = form{
      if result == None {
        return Err("Form with the provided id doesn't exist 🙂")
      }
    }
  }


  let results = insert_doc(&client.client, "crabs_test", "selects", &select).await.expect("Skip");

  if let Some(form_id) = &select.form_id{
    let document = doc! { "$push": { "selects": trim_quotes(&results.inserted_id.to_string()) } };
    update_push::<Form>(&client.client, "crabs_test", "forms", document, form_id).await;
  }

  Ok("Select Added successfully 🙂")
}

