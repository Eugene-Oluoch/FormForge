use mongodb::{Client, bson::{doc, self, from_bson}};
use rocket::serde::json::Json;
use crate::{models::{
  select::{Select,SelectReceive},
  form::{Form}, traits::ResetDefaults,
  option::{OptionSelect}
}, utils::{ReturnId, trim_quotes, ReturnError, ReturnMessage, ReturnErrors,update_document,update_form_id_cases}};
use crate::db::{
  get_all,
  get_by_id,
  insert_doc,
  update_one,
  update_many
};
use crate::repository::{
  map
};
use crate::views::{
  options::{add_option_helper}
};


pub async fn get_select_view(id:String,client:&Client) -> Result<Json<SelectReceive>,Json<ReturnError>>{
  // REASON FOR DOUBLE QUERY IS THAT AGGREATE MONGO QUERY THROW ERROR IF ID DOESN'T MATCH 
  let validate_if_it_exists = get_by_id::<Select>(client, "selects", &id).await.expect("Failed");
  if validate_if_it_exists.is_none(){
    return Err(Json(ReturnError::new("Select with the given id doesn't exist 🙁")));
  }


  let document = get_all::<Select>(client, "selects", map("select",id.as_str())).await;
  if let Ok(doc) = document{
    let results:SelectReceive = from_bson(bson::Bson::Document(doc)).expect("Failed here");
    if results.archive == Some(true){
      Err(Json(ReturnError::new("Select with the given id doesn't exist 🙁")))
    }else{
      Ok(Json(results))
    }
  } else {
    Err(Json(ReturnError::new("Select with the given id doesn't exist 🙁")))
  }
}

pub async fn add_select_helper<'a>(select:&'a mut SelectReceive,client:&'a Client) -> Result<String,&'a str> {
    // RESET AND SET ID
    let _ = &mut select.reset();
  
    // VALIDATION FOR FORM ID
    if let Some(form_id) = &select.form_id{
      let form = get_by_id::<Form>(client,"forms", form_id.as_str()).await;
      if let Ok(result) = form{
        if result == None {
          return Err("Form with the provided id doesn't exist 🙂")
        }
      }
    }
  
    let results = insert_doc(client, "selects", &select.convert()).await.expect("Skip").inserted_id.to_string();
  
    // CREATE THE OPTIONS -> PLANNING TO MAKE THIS A MULTI-THREAD 
    if let Some(options) = &mut select.options{
      for option in options{
        option.select_id = Some(results.as_str().clone().to_string().trim_matches('"').to_string());
        let _ = add_option_helper(option, client).await;
      }
    }
  
  
    if let Some(form_id) = &select.form_id{
      let document = doc! { "$push": { "selects": trim_quotes(&results) } };
      let _ = update_one::<Form>(client, "forms", document, form_id).await;
    }
  
    Ok(trim_quotes(&results))
}

pub async fn add_select_view(data:Json<SelectReceive>,client:&Client) -> Result<Json<ReturnId>, Json<ReturnError>>{
  let mut select = data.0;
  
  let results = add_select_helper(&mut select, client).await;
  if let Ok(id) = results{
    Ok(Json(ReturnId::new(&id)))
  }else{
    Err(Json(ReturnError::new("Failed")))
  }
}


pub async fn delete_select_view<'a>(id:&str,client:&Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  let update = doc! { "$set": {"archive":true} };
  let results = update_one::<Select>(client,"selects", update,id).await;

  

  if let Ok(_) = &results{

    // ARCHIVE SELECT'S OPTIONS
    update_many::<OptionSelect>(client, "options", doc! {"select_id":id}, doc! {"$set":{"archive":true}}).await;


    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  }else {
    Err(Json(ReturnError::new("Failed to delete 🙁")))
  }
}


// VALIDATE REQUIRED FIELD
pub async fn update_select_view<'a>(id:&'a str,select:SelectReceive,client:&'a Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnErrors>>{

  // TODO VALIDATE SELECT EXISTS
  let mut select_results:Select = get_by_id::<Select>(client, "selects", &id).await.expect("Failed").unwrap();

  // CHECK IF SELECT IS ARCHIVED
  if let Some(val) = &select_results.archive{
    if *val == true{
      return Err(Json(ReturnErrors::new(["Select with the provided id doesn't exists 🙁".to_string()].to_vec())));
    }
  }

  // UPATED UPDATED AT
  let _ = &mut select_results.update();


  // HANDLE FORM ID UPDATE CASES
  let form_id_results = update_form_id_cases(&select_results.form_id, &select.form_id, client, id,"selects").await;
  if let Err(err) = form_id_results{
    return Err(err);
  }

  if let Ok(data) = form_id_results{
    if let Some(id) = data{
      if id.to_lowercase() != "ignore".to_string().to_lowercase(){
        select_results.form_id = Some(id);
      }
    }else{
      select_results.form_id = None;
    }
  }


  // UPDATE THE REST
  select_results.multiple = select.multiple;
  select_results.size = select.size;
  select_results.required = select.required;
  select_results.step = select.step;


  update_document::<Select>(&select_results, id, "selects", client).await

}