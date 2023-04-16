use mongodb::{Client, bson::{doc, self, from_bson}};
use rocket::serde::json::Json;
use crate::{models::{
  select::{Select,SelectReceive},
  form::{Form}, traits::ResetDefaults
}, utils::{ReturnId, trim_quotes, ReturnError, ReturnMessage}};
use crate::db::{
  get_all,
  get_by_id,
  insert_doc,
  update_one
};
use crate::repository::{
  map
};
use crate::views::{
  options::{add_option_helper}
};

pub async fn get_select_view(id:String,client:&Client) -> Result<Json<SelectReceive>,Json<ReturnError>>{
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
    for option in &mut select.options{
      option.select_id = Some(results.as_str().clone().to_string().trim_matches('"').to_string());
      let _ = add_option_helper(option, client).await;
    }
  
  
    if let Some(form_id) = &select.form_id{
      let document = doc! { "$push": { "selects": trim_quotes(&results) } };
      update_one::<Form>(client, "forms", document, form_id).await;
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
    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  }else {
    Err(Json(ReturnError::new("Failed to delete 🙁")))
  }
}