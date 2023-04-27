use mongodb::{Client, bson::doc};
use rocket::serde::json::Json;

use crate::{
  db::{
    get_by_id,
    insert_doc,
    update_one
  },
  models::{
    textarea::{TextArea}, traits::ResetDefaults, form::Form
  },
  utils::{
    ReturnError,
    ReturnMessage,
    trim_quotes,
    ReturnId,
    ReturnErrors,
    update_form_id_cases,
    update_document
  }
};



pub async fn get_textarea_view<'a>(id:&'a str,client:&'a Client) -> Result<Json<TextArea>,Json<ReturnError<'a>>> {
  
  let results = get_by_id::<TextArea>(client, "textareas", id).await.expect("Failed");
  if let Some(val) = results{
    if let Some(arc) = &val.archive{
      if arc == &true{
        return Err(Json(ReturnError::new("Textarea with the given id doesn't exist 🙁")));
      }
    }
    
    Ok(Json(val))
    
  }else{
    Err(Json(ReturnError::new("Textarea with the provided id doesn't exists 🙁")))
  }
}

pub async fn add_textarea_alone(data:&mut TextArea,client:&Client)->String{
  data.reset();
  insert_doc(client, "textareas", &data).await.expect("failed").inserted_id.to_string()
}

pub async fn add_textarea_helper<'a>(textarea:&mut TextArea,client:&Client) -> Result<String,&'a str> {
  if let Some(form_id) = &textarea.form_id{
    let form = get_by_id::<Form>(client,"forms", form_id.as_str()).await;
    if let Ok(result) = form{
      if result == None {
        return Err("Form with the provided id doesn't exist 🙂")
      }
    }
  }

  let results = add_textarea_alone(textarea, client).await;


  if let Some(form_id) = &textarea.form_id{
    let document = doc! { "$push": { "textareas": trim_quotes(&results) } };
    let _ = update_one::<Form>(client, "forms", document, form_id).await;
  }

  Ok(trim_quotes(&results))
}


pub async fn add_textarea_view(data:Json<TextArea>,client:&Client) -> Result<Json<ReturnId>, Json<ReturnError>>{
  let mut textarea = data.0;
  
  let results = add_textarea_helper(&mut textarea, client).await;
  if let Ok(id) = results{
    Ok(Json(ReturnId::new(&id)))
  }else{
    Err(Json(ReturnError::new("Failed")))
  }
}


pub async fn delete_textarea_view<'a>(id:&str,client:&Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  let update = doc! { "$set": {"archive":true} };
  let results = update_one::<TextArea>(client,"textareas", update,id).await;

  if let Ok(_) = &results{
    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  }else {
    Err(Json(ReturnError::new("Failed to delete 🙁")))
  }
}


pub async fn update_textareas_view<'a>(id:&'a str,textarea:TextArea,client:&'a Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnErrors>>{

  // TODO VALIDATE SELECT EXISTS
  let mut textarea_results:TextArea = get_by_id::<TextArea>(client, "textareas", &id).await.expect("Failed").unwrap();

  // CHECK IF SELECT IS ARCHIVED
  if let Some(val) = &textarea_results.archive{
    if *val == true{
      return Err(Json(ReturnErrors::new(["Textarea with the provided id doesn't exists 🙁".to_string()].to_vec())));
    }
  }

  // UPATED UPDATED AT
  let _ = &mut textarea_results.update();


  // HANDLE FORM ID UPDATE CASES
  let form_id_results = update_form_id_cases(&textarea_results.form_id, &textarea.form_id, client, id,"textareas").await;
  if let Err(err) = form_id_results{
    return Err(err);
  }

  if let Ok(data) = form_id_results{
    if let Some(id) = data{
      if id.to_lowercase() != "ignore".to_string().to_lowercase(){
        textarea_results.form_id = Some(id);
      }
    }else{
      textarea_results.form_id = None;
    }
  }


  // UPDATE THE REST
  textarea_results.rows = textarea.rows;
  textarea_results.cols = textarea.cols;
  textarea_results.required = if textarea.required.is_some() {textarea.required} else {textarea_results.required};

  println!("{:?}",textarea_results);

  update_document::<TextArea>(&textarea_results, id, "textareas", client).await

}