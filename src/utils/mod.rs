use rocket::serde::json::Json;
use serde::{Serialize, Deserialize};
use mongodb::{Client, bson::{to_bson, doc}};
use chrono::Utc;

use crate::{db::{update_one, get_by_id}, models::form::Form};




pub fn trim_quotes(string:&String) -> String{
  string.trim_matches('"').to_string()
}


pub fn generate_current_time() -> i64{
  Utc::now().timestamp_millis()
}

#[derive(Clone)]
pub struct StateCustom{
  pub client:Client
}

impl StateCustom {
  pub fn new(client:Client) -> Self{
    Self {
      client:client
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnId {
  id:String
}

impl  ReturnId  {
  pub fn new(id:&str) -> Self{
    Self {
      id:id.to_string()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnError<'a>{
  error:&'a str
}

impl <'a> ReturnError <'a> {
  pub fn new(error:&'a str) -> Self{
    Self {
      error
    }
  }
}
#[derive(Serialize,Deserialize,Debug)]
pub struct ReturnErrors{
  pub errors:Vec<String>
}

impl ReturnErrors{
  pub fn new(errors:Vec<String>) -> Self{
    Self{
      errors
    }
  }
}

#[derive(Serialize,Deserialize,Debug)]
pub struct ReturnMessage<'a>{
  message:&'a str
}

impl <'a> ReturnMessage <'a>{
  pub fn new(message:&'a str) -> Self{
    Self {
      message
    }
  }
}
 

pub async fn update_document<'a,T>(document:&T,id:&str,db_collenction:&str,client:&Client) -> Result<Json<ReturnMessage<'a>>, Json<ReturnErrors>>
where T:Serialize
{
  let bson = to_bson(document).unwrap();
  let update_query = doc! {"$set":bson.as_document().unwrap().to_owned()};
  let results = update_one::<T>(client, db_collenction, update_query, id).await;
  match &results {
    Ok(_) => Ok(Json(ReturnMessage::new("Updated successfully 🙂"))),
    Err(_) => Err(Json(ReturnErrors::new(["Failed to update 🙁".to_string()].to_vec())))
  }
}

pub async fn update_remove_from_form(id:&str,id1:&str,field:&str,client:&Client){
  let delete_query = doc! {"$pullAll":{field:[id]}};
  let _ = update_one::<Form>(client, "forms", delete_query, &id1).await;
}

pub async fn update_add_to_form(id:&str,id2:&str,field:&str,client:&Client){
  let update_query = doc! {"$push":{field:&id}};
  let _ = update_one::<Form>(client, "forms", update_query, &id2).await;
}

pub async fn update_form_id_cases(to_compare:&Option<String>,data:&Option<String>,client:&Client,id:&str,field:&str) -> Result<Option<String>, Json<ReturnErrors>>{
  if let (Some(id1),Some(id2)) = (to_compare,data){
    if id1 != id2{
      // VALIDATE PROVIDED FORM ID
      let form = get_by_id::<Form>(client, "forms", &id2).await.expect("Failed");
      if form.is_none(){
        return Err(Json(ReturnErrors::new(["Form with the provided form id doesn't exists 🙁".to_string()].to_vec())));
      }

      //UPDATE PREVIOUS FORM
      let _ = update_remove_from_form(&id, &id1, &field, client).await;


      // UPDATE CURRENT FORM_ID
      let _ = update_add_to_form(&id,&id2,&field,client).await;
      return Ok(data.clone());


    }
  } else if data.is_some() && to_compare.is_none(){
    let form_id = data.as_ref().unwrap();
    let form = get_by_id::<Form>(client, "forms", &form_id).await.expect("Failed");
    if form.is_none(){
      return Err(Json(ReturnErrors::new(["Form with the provided form id doesn't exists 🙁".to_string()].to_vec())));
    }

    //UPDATE FORM
    let _ = update_add_to_form(&id,&form_id,&field,client).await;
    
    // UPDATE DATA TO SEND
    return Ok(Some(form_id.to_string()));

  } else if data.is_none() && to_compare.is_some(){
    // UPDATE PREVIOUS FORM
    let _ = update_remove_from_form(&id, &to_compare.as_ref().unwrap(), &field, client).await;

    return Ok(None);
  }

  Ok(Some("ignore".to_string()))
}