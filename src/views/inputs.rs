use mongodb::{Client, bson::doc};
use rocket::serde::json::Json;
use crate::{
  db::{
    get_by_id,
    insert_doc,
    update_one
  },
  models::{
    input::{Input},
    form::{Form}, traits::ResetDefaults
  },
  utils::{
    ReturnError,
    ReturnId,
    trim_quotes,
    ReturnErrors,
    ReturnMessage
  }
};



pub async fn get_input_view<'a>(id:&str,client:&Client) -> Result<Json<Input>,Json<ReturnError<'a>>>{
  let input_data = get_by_id::<Input>(client, "inputs", id).await.expect("Failed");
  if let Some(result) = input_data{
    if result.archive == Some(true){
      Err(Json(ReturnError::new("Input with the provided id doesn't exists. 🙁"))) 
    }else{
      Ok(Json(result))
    }
  }else{
    Err(Json(ReturnError::new("Input with the provided id doesn't exists. 🙁")))
  }
}

pub async fn validate(input:&Input) -> ReturnErrors{
  let mut errors:Vec<String> = vec![];


  if input.type_identifier == None{
    errors.push("Type identifier is required!".to_string());
  }

  if input.name == None{
    errors.push("Name is required!".to_string());
  }

  ReturnErrors::new(errors)

}

pub async fn add_input_helper<'a>(input:&'a mut Input,client:&'a Client) -> Result<String,&'a str> {
  let _ = input.reset();
  let _ = input.map_type();

  // FORM ID 
  if let Some(form) = &input.form_id{
    let form_result = get_by_id::<Form>(client,"forms", form).await.expect("Failed");
    if form_result == None {
      return Err("Form with the provided id doesn't exists 🙁")
    }
  }
  let input_id = insert_doc(client,"inputs", &input).await.unwrap().inserted_id.to_string();

  // UPDATE FORM'S Inputs
  if let Some(form) = &input.form_id{
    let document = doc! { "$push": { "inputs": &input_id.trim_matches('"').to_string() } };
    let _ = update_one::<Form>(client,"forms", document, form).await;
  }

  Ok(trim_quotes(&input_id))

}

pub async fn add_input_view(data:Json<Input>,client:&Client) -> Result<Json<ReturnId>,Json<ReturnErrors>> {
  let mut input = data.0;
  let results = add_input_helper(&mut input, client).await;
  if let Ok(id) = results{
    Ok(Json(ReturnId::new(&id)))
  }else{
    Err(Json(ReturnErrors::new(["Form with the provided id doesn't exists 🙁".to_string()].to_vec())))
  }
  
}

pub async fn delete_input_view<'a> (id:&str,client:&Client) -> Result<Json<ReturnMessage<'a>>,Json<ReturnError<'a>>>{
  let update = doc! {"$set":{"archive":true}};
  let results = update_one::<Input>(client, "inputs", update, id).await;
  if let Ok(_) = results{
    Ok(Json(ReturnMessage::new("Deleted successfully 🙂")))
  } else {
    Err(Json(ReturnError::new("Sorry its not you its us 🙁")))
  }
}