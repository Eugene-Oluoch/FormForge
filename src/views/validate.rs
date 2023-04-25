use std::{collections::{HashMap}, sync::{Arc, Mutex}, cell::RefCell};

use mongodb::Client;

use crate::{
  models::{
    validate::{Validate}, 
    form::Form,
    input::{Input},
    select::{Select}
  }, db::get_by_id
};

pub enum Collection{
  Selects,
  Inputs
}

pub async fn validate_id_for_form(errors:Arc<Mutex<RefCell<Vec<String>>>>,user:Arc<HashMap<String,String>>,form:Arc<Vec<String>>,collection:Collection,client:&Client){

  for id in user.keys(){
    if form.contains(id){

      match collection{
        Collection::Inputs => {
          let field = tokio::join!(get_by_id::<Input>(client, "inputs", id)).0.expect("Failed").unwrap();
        },
        Collection::Selects => {
          let field = tokio::join!(get_by_id::<Select>(client, "selects", id)).0.expect("Failed").unwrap();
          
        }
      }
    }else{
      errors.as_ref().lock().expect("Failed").borrow_mut().push(id.clone());
    }
  }

}

pub async fn validate_data_view(valid:Validate,client:&Client){
  // VALIDATE IF FORM EXISTS
  let form_results = get_by_id::<Form>(client, "forms", &valid.form_id.as_ref().unwrap()).await.expect("failed");
  if let Some(form) = form_results{
    let (user_inputs_ids,user_selects_ids) = (Arc::new(valid.inputs.as_ref().unwrap().clone()),Arc::new(valid.selects.as_ref().unwrap().clone()));
    let (form_inputs_ids,form_selects_ids) = (Arc::new(form.inputs.as_ref().unwrap().clone()),Arc::new(form.selects.as_ref().unwrap().clone()));

  


    // HELP SHARE CLIENT ACROSS THREADS
    let (client_clone,client_clone2) = (Arc::new(client.clone()),Arc::new(client.clone()));
  
    // TO HELP TRACK ID THAT AREN'T VALID
    let errors:Arc<Mutex<RefCell<Vec<String>>>> = Arc::new(Mutex::new(RefCell::new(Vec::new())));

    let (errors_clone,errors_clone2) = (Arc::clone(&errors),Arc::clone(&errors));


    let input_valid = tokio::spawn(
      async move  {
        let _ = validate_id_for_form(
          errors_clone,
          user_inputs_ids,
          form_inputs_ids,
          Collection::Inputs,
          &*client_clone,
        )
        .await;
    });

    let select_valid = tokio::spawn(async move {
      let _ = validate_id_for_form(
          errors_clone2,
          user_selects_ids,
          form_selects_ids,
          Collection::Selects,
          &*client_clone2,
      )
      .await;
    });

    let _ = tokio::join!(select_valid, input_valid);

    println!("{:?}",errors);





  }else{
    println!("It doesn't exist");
  }

}