use std::{thread, sync::{Arc}};

use mongodb::{Client, bson::{from_bson, self}};
use rocket::serde::json::Json;
use tokio::sync::watch::error;

use crate::{
  db::{
    get_all,
    insert_doc
  },
  models::{
    form::{Form,FormReceive}, traits::ResetDefaults, select::SelectReceive
  },
  utils::{
    ReturnMessage,
    ReturnId,
    trim_quotes, ReturnErrors
  },
  repository::{
    map
  },
  views::{
    selects::{
      add_select_helper
    },
    inputs::{
      add_input_helper
    }
  }
};






pub async fn get_form_view<'a>(id:String, client:&Client) -> Result<Json<FormReceive>,Json<ReturnMessage<'a>>>{
  let results = get_all::<Form>(client,"forms", map("form",id.as_str())).await;
  if let Ok(result) = results{
    let mut final_result:FormReceive = from_bson(bson::Bson::Document(result)).expect("failed");

    // RESET SELECTS IF MONGO RETURN A NONE RECORD
    if let Some(selects) = &final_result.selects{
      if selects.len() == 1 && selects[0]._id == None{
        let reset_selects:Vec<SelectReceive> = Vec::new();
        final_result.selects = Some(reset_selects);
      }
    }

    if final_result.archive == Some(true){
      Err(Json(ReturnMessage::new("Failed to get the form 🙁"))) 
    }else{
      Ok(Json(final_result))
    }

  } else {
    Err(Json(ReturnMessage::new("Failed to get the form 🙁")))
  }
}


pub async fn validate(form:&FormReceive)-> Option<ReturnErrors>{

  if form.name == None{
    Some(ReturnErrors::new(["Name is required!".to_string()].to_vec()))
  }else{
    None
  }

}

pub async fn add_form_view(data:Json<FormReceive>,client:&Client) -> Json<ReturnId>{
  let mut form = data.0;

  // GENERATE A RANDOM ID FOR FORM
  let mut form2 = form.convert(Some(Vec::new()),Some(Vec::new()));
  let _ = &mut form2.reset();



  // ID OF CREATED FORM
  let form_id =trim_quotes(&insert_doc(client, "forms", &form2).await.unwrap().inserted_id.to_string());

  // FORM_ID REFERNCE CLONE TO PASS TO THREADS
  let form_id_clone = Arc::new(form_id.clone());
  let form_id_clone2 = Arc::clone(&form_id_clone);

  // DB_CONNECTION REFERENCE CLONE TO PASS TO THREADS
  let (client_clone,client_clone2) = (Arc::new(client.clone()),Arc::new(client.clone()));
  
  // THREADS TO HANDLE SELECTS AND OPTION CREATION
  let selects_creation = thread::spawn({
      move || {
        if let Some(selects) = &mut form.selects{
          for select in selects.iter_mut() {
              select.form_id = Some(form_id_clone.to_string());
              let _ = tokio::runtime::Runtime::new()
                  .unwrap()
                  .block_on(add_select_helper(select, &*client_clone));
          }
        }
      }
    });
  
  let inputs_creation = thread::spawn({
    move || {
      if let Some(inputs) = &mut form.inputs{
        for input in inputs.iter_mut() {
            input.form_id = Some(form_id_clone2.to_string());
            let _ = tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(add_input_helper(input, &*client_clone2));
        }
      }
    }
  });
  
  let threads = vec![selects_creation,inputs_creation];
  for t in threads.into_iter(){
    let _ = t.join();
  }
  


  Json(ReturnId::new(&form_id.to_string()))
}