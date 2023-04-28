use std::{sync::{Arc}, collections::HashMap, vec};

use mongodb::{Client, bson::{from_bson, self, doc}};
use rocket::{serde::json::Json};
use tokio::{task::JoinHandle};

use crate::{
  db::{
    get_all,
    insert_doc,
    update_many,
    update_one, get_by_id
  },
  models::{
    form::{Form,FormReceive,FormReceiveFinal}, 
    traits::ResetDefaults, 
    select::{SelectReceive, Select},
    input::{Input}
  },
  utils::{
    ReturnMessage,
    ReturnId,
    trim_quotes, 
    ReturnErrors,
    ReturnError,
    update_document
  },
  repository::{
    map
  },
  views::{
    selects::{
      add_select_alone
    },
    inputs::{
      add_input_alone
    },
    textarea::{
      add_textarea_alone
    }
  }
};



pub fn push_to_hashmap(type_:&str,hashmap_:&mut HashMap<String, Vec<Input>>,inputs:&Input){
    if hashmap_.contains_key(type_){
      hashmap_.get_mut(type_).unwrap().push(inputs.clone());
    }else{
      hashmap_.insert(type_.to_string(), vec![inputs.clone()]);
    } 
}


pub async fn get_form_view<'a>(id:String, client:&Client) -> Result<Json<FormReceiveFinal>,Json<ReturnErrors>>{
  // REASON FOR DOUBLE QUERY IS THAT AGGREATE MONGO QUERY THROW ERROR IF ID DOESN'T MATCH 
  let validate_if_it_exists = get_by_id::<Form>(client, "forms", &id).await.expect("Failed");
  if validate_if_it_exists.is_none(){
    return Err(Json(ReturnErrors::new(["Form with the given id doesn't exist 🙁".to_string()].to_vec())));
  }

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
      Err(Json(ReturnErrors::new(["Failed to get the form 🙁".to_string()].to_vec()))) 
    }else{


      // GROUP CHECKBOX AND RADIO THAT HAVE SIMILIAR NAMES
      let mut checkboxes_groups:HashMap<String, Vec<Input>> = HashMap::new();
      let mut radios_groups:HashMap<String,Vec<Input>> = HashMap::new();
      let mut filtered_inputs = Vec::new();
      
      for inputs in final_result.inputs.as_ref().unwrap(){
        if inputs.type_identifier == Some("checkbox".to_string()){
          if let Some(n) = &inputs.name{
            push_to_hashmap(n.as_str(),&mut checkboxes_groups,inputs);
          }
        }else if inputs.type_identifier == Some("radio".to_string()){
          if let Some(n) = &inputs.name{
            push_to_hashmap(n.as_str(),&mut radios_groups,inputs);
          }
        }else{
          filtered_inputs.push(inputs.clone());
        }
      }

      Ok(Json(final_result.to_final(Some(filtered_inputs),checkboxes_groups, radios_groups).await))
    }

  } else {
    Err(Json(ReturnErrors::new(["Failed to get the form 🙁".to_string()].to_vec())))
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
  let mut form2 = form.convert(Some(Vec::new()),Some(Vec::new()),Some(Vec::new()));
  let _ = &mut form2.reset();



  // ID OF CREATED FORM
  let form_id =trim_quotes(&insert_doc(client, "forms", &form2).await.unwrap().inserted_id.to_string());

  // FORM_ID REFERNCE CLONE TO PASS TO THREADS
  let form_id_selects = Arc::new(form_id.clone());
  let (form_id_inputs,form_id_textareas) = (Arc::clone(&form_id_selects),Arc::clone(&form_id_selects));

  // DB_CONNECTION REFERENCE CLONE TO PASS TO THREADS
  let (selects_client,inputs_client,textareas_client) = (Arc::new(client.clone()),Arc::new(client.clone()),Arc::new(client.clone()));

  // THREADS TO HANDLE SELECTS AND OPTION CREATION
  let selects_creation:JoinHandle<Vec<String>> = tokio::spawn(
    async move {
      let mut selects_id:Vec<String> = Vec::new();
      if let Some(selects) = &mut form.selects{
        for select in selects.iter_mut() {
            select.form_id = Some(form_id_selects.to_string());
            let select_ids = add_select_alone(select, &*selects_client).await;
            selects_id.push(trim_quotes(&select_ids));
        }
      }
      selects_id
    }
  );

  let inputs_creation:JoinHandle<Vec<String>> = tokio::spawn(
    async move  {
      let mut inputs_id:Vec<String> = Vec::new();
      if let Some(inputs) = &mut form.inputs{
        for input in inputs.iter_mut() {
            input.form_id = Some(form_id_inputs.to_string());
            let input_ids = add_input_alone(input, &*inputs_client).await;
            inputs_id.push(trim_quotes(&input_ids));
        }
      }
      inputs_id
    }
  );


  let textarea_creation:JoinHandle<Vec<String>> = tokio::spawn(
    async move  {
      let mut textareas_id:Vec<String> = Vec::new();
      if let Some(textareas) = &mut form.textareas{
        for textarea in textareas.iter_mut() {
          textarea.form_id = Some(form_id_textareas.to_string());
            let input_ids = add_textarea_alone(textarea, &*textareas_client).await;
            textareas_id.push(trim_quotes(&input_ids));
        }
      }
      textareas_id
    }
  );

  
  let ids_from_threads = tokio::join!(selects_creation,inputs_creation,textarea_creation);
  
  // UPDATE FORM WITH INPUT AND SELECT IDS
  let document = doc! { 
    "$push": { 
      "selects": {"$each": ids_from_threads.0.expect("failed")}, 
      "inputs":{"$each": ids_from_threads.1.expect("failed")},
      "textareas":{"$each": ids_from_threads.2.expect("failed")},
    } 
  };
  let _ = update_one::<Form>(client,"forms", document, &form_id).await;

  


  Json(ReturnId::new(&form_id.to_string()))
}

pub async fn update_form_view<'a>(id:&str,form:FormReceive,client:&Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnErrors>>{

  // VALIDATE IF FORM EXISTS
  let mut form_results = get_by_id::<Form>(client, "forms", &id).await.expect("Failed").unwrap();

  // CHECK IF FORM IS ARCHIVED
  if let Some(val) = &form_results.archive{
    if *val == true{
      return Err(Json(ReturnErrors::new(["Form with the provided id doesn't exists 🙁".to_string()].to_vec())));
    }
  }

  // UPDATED AT UPDATE
  let _ = &mut form_results.update();


  // UPDATE FIELD VALIDATE REQUIRED FIELD
  form_results.name = form.name;
  form_results.steps = form.steps;


  update_document::<Form>(&form_results, id, "forms", client).await

}



pub async fn delete_form_view<'a>(id:&'a str,client:&'a Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{

  let update = doc! { "$set": {"archive":true} };
  let results = update_one::<Form>(client,"forms", update,id).await;

  

  if let Ok(_) = &results{

    // ARCHIVE FORM'S INPUTS
    update_many::<Input>(client, "inputs", doc! {"form_id":id}, doc! {"$set":{"archive":true}}).await;
    
    // ARCHIVE FORM'S SELECTS(NOTE: not handled options nested inside the selects)
    update_many::<Select>(client, "selects", doc! {"form_id":id}, doc! {"$set":{"archive":true}}).await;

    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  }else {
    Err(Json(ReturnError::new("Failed to delete 🙁")))
  }

}