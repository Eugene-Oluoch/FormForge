use mongodb::{Client, bson::{doc, to_bson}};
use rocket::serde::json::Json;

use crate::{
  models::{
  option::{OptionSelect},
  select::{Select}, 
  traits::{ResetDefaults}
}, 
utils::{
  ReturnId, 
  ReturnError,
  ReturnMessage,
  ReturnErrors
},
db::{
  get_by_id,
  insert_doc,
  update_one
}
};


pub async fn validate(option:&OptionSelect)->Option<ReturnErrors>{
  if option.name == None{
    Some(ReturnErrors::new(["Name is required!".to_string()].to_vec()))
  }else{
    None
  }
}

pub async fn get_option_view<'a>(id:&str,client:&Client) -> Result<Json<OptionSelect>,Json<ReturnErrors>>{

  let option_data = get_by_id::<OptionSelect>(client,"options",id).await.expect("failed");
  if let Some(result) = option_data{
    if result.archive == Some(true){
      Err(Json(ReturnErrors::new(["Option with the provided id doesn't exists.".to_string()].to_vec())))
    }else{
      Ok(Json(result))
    }
  }else{
    Err(Json(ReturnErrors::new(["Option with the provided id doesn't exists.".to_string()].to_vec())))
  }
}

pub async fn add_option_helper(option:&mut OptionSelect,client:&Client) -> Result<String,String>{
  // RESET AND ASSIGN ID
  let _ = option.reset();

  // VALIDATE SELECT ID 
  if let Some(select_id) = &option.select_id{
    let select_requested = get_by_id::<Select>(client,"selects",select_id.as_str()).await.expect("Failed on db level");
    if select_requested == None{
      return Err("Select with the given id doesn't exist".to_string())
    }
  }

  // ID OF CREATED OPTION
  let option_id = insert_doc(client, "options", &option).await.unwrap().inserted_id.to_string();

  // UPDATE OPTIONS IN THE SELECT
  if let Some(select_id) = &option.select_id{
    let document = doc! { "$push": { "options": &option_id.trim_matches('"').to_string() } };
    let _ = update_one::<Select>(client,"selects", document, select_id).await;
  }

  Ok(option_id.trim_matches('"').to_string())

}

pub async fn add_option_view(data:OptionSelect,client:&Client) -> Result<Json<ReturnId>,Json<ReturnErrors>> {
  let mut option = data;
  let results = add_option_helper(&mut option, client).await;
  if let Ok(result) = results{
    Ok(Json(ReturnId::new(result.as_str())))
  }else{
    Err(Json(ReturnErrors::new(["Failed".to_string()].to_vec())))
  }
}


pub async fn delete_option_view<'a>(id:&str,client:&Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  let update_query = doc! {"$set":{"archive":true}};
  let results = update_one::<OptionSelect>(client, "options",update_query,id).await;
  if let Ok(_) = &results{
    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  }else {
    Err(Json(ReturnError::new("Failed to delete 🙁")))
  }

}



pub async fn update_remove_from_select(id:&str,select_id:&str,client:&Client){
  let select_remove_option = doc! {"$pullAll":{"options":[id]}};
  let _ = update_one::<Select>(client, "selects", select_remove_option, select_id).await;
}

pub async fn update_add_to_select(id:&str,select_id:&str,client:&Client){
  let select_update = doc! {"$push":{"options":&id}};
  let _ = update_one::<Select>(client, "selects", select_update, select_id).await;
}


pub async fn update_option_view<'a>(id:&'a str,mut data:OptionSelect,client:&'a Client,) -> Result<Json<ReturnMessage<'a>>,Json<ReturnErrors>>{

  let option = get_by_id::<OptionSelect>(client, "options", id).await.expect("failed").unwrap();

  // UPDATES UPDATE AT
  data.update();
  data._id = option._id;
  data.created_at = option.created_at;

  // TODO HANDLE WHEN SELECT ID CHANGES

  if let (Some(s_id),Some(s_id2)) = (&option.select_id,&data.select_id){
    if s_id != s_id2{
      // Update the select that is in option and update the new select
      // Valid if the provided select exists
      let new_select = get_by_id::<Select>(client, "selects", &s_id2).await.expect("failed");
      if new_select.is_none(){
        return Err(Json(ReturnErrors::new(["Select with the provided select id doesn't exist🙁".to_string()].to_vec())))
      }

      // UPDATE PREVIOUS SELECT
      let _ = update_remove_from_select(&id, &s_id, client).await;

      // UPDATE NEWLY SELECTED SELECT
      let _ = update_add_to_select(&id, &s_id2, client).await;

    }
  }
  else{
    if option.select_id.is_none() && data.select_id.is_some(){
      // VALIDATE IF THE PROVIDED SELECT EXISTS
      if let Some(s_id) = &data.select_id{
        let new_select = get_by_id::<Select>(client, "selects", &s_id).await.expect("failed");
        if new_select.is_none(){
          return Err(Json(ReturnErrors::new(["Select with the provided select id doesn't exist🙁".to_string()].to_vec())))
        }

        // UPDATE NEWLY SELECTED SELECT
        let _ = update_add_to_select(&id, &s_id, client).await;
      }

    }else if option.select_id.is_some() && data.select_id.is_none(){
      // REMOVE OPTION FROM SELECT
      let _ = update_remove_from_select(&id,&option.select_id.unwrap(), client).await;
      // UPDATE DATA 
      data.select_id = None;

    }
  }

  let bson = to_bson(&data).unwrap();
  let update_query = doc! {"$set":bson.as_document().unwrap().to_owned()};
  let results = update_one::<OptionSelect>(client, "options", update_query, id).await;
  match &results {
    Ok(_) => Ok(Json(ReturnMessage::new("Updated successfully 🙂"))),
    Err(_) => Err(Json(ReturnErrors::new(["Failed to update 🙁".to_string()].to_vec())))
  }

}