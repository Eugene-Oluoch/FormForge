use crate::utils::{ReturnErrors,ReturnError, StateCustom,ReturnId,ReturnMessage};
use crate::OptionSelect;
use mongodb::bson::doc;
use rocket::State;
use rocket::serde::json::Json;
use crate::views::{
  options::{
    add_option_view,
    get_option_view,
    update_option_view,
    delete_option_view,
    validate
  }
};


// TODO MAP CORRECT STATUS CODES
#[get("/<id>")]
pub async fn get_option_by_id<'a>(id:&'a str,client:&'a State<StateCustom>) -> Result<Json<OptionSelect>,Json<ReturnErrors>>{
  get_option_view(id, &client.client).await
}


#[post("/add",data="<data>")]
pub async fn add_option(data:Json<OptionSelect>,client:&State<StateCustom>) -> Result<Json<ReturnId>,Json<ReturnErrors>>{
  if let Some(error) = validate(&data.0).await{
    Err(Json(error))
  }else{
    add_option_view(data, &client.client).await
  }
}

#[put("/<id>",data="<data>")]
pub async fn update_option<'a>(id:&'a str,data:Json<OptionSelect>,client:&'a State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  let option = data.0;
  update_option_view(id, option, &client.client).await
}



#[delete("/<id>")]
pub async fn delete_option<'a>(id:&str,client:&State<StateCustom>) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  delete_option_view(id, &client.client).await
}



// DELETE OPTION FROM ALL SELECTS
// let match_selects = doc! {"options":id};
// let delete =doc! {"$pullAll":{"options":[id]}};
// update_many::<Select>(&client.client, "crabs_test", "selects", match_selects, delete, id).await;
