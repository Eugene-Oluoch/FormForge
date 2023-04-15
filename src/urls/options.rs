use crate::utils::{ReturnError, StateCustom,ReturnId,ReturnMessage};
use crate::OptionSelect;
use mongodb::bson::doc;
use rocket::State;
use rocket::serde::json::Json;
use crate::views::{
  options::{
    add_option_view,
    get_option_view,
    delete_option_view
  }
};


// TODO MAP CORRECT STATUS CODES
#[get("/<id>")]
pub async fn get_option_by_id<'a>(id:&'a str,client:&'a State<StateCustom>) -> Result<Json<OptionSelect>,Json<ReturnError<'a>>>{
  get_option_view(id, &client.client).await
}


#[post("/add",data="<data>")]
pub async fn add_option(data:Json<OptionSelect>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnError>>{
  add_option_view(data, &client.client).await
}


#[delete("/<id>")]
pub async fn delete_option<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  delete_option_view(id, &client.client).await
}



// DELETE OPTION FROM ALL SELECTS
// let match_selects = doc! {"options":id};
// let delete =doc! {"$pullAll":{"options":[id]}};
// update_many::<Select>(&client.client, "crabs_test", "selects", match_selects, delete, id).await;
