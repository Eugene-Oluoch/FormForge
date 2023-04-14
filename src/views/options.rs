use mongodb::{Client, bson::doc};

use crate::models::{
  option::{OptionSelect},
  select::{Select}, 
  traits::{ResetDefaults}
};
use crate::db::{
  get_by_id,
  insert_doc,
  update_push
};

pub async fn add_option_view(option:&mut OptionSelect,client:&Client) -> Result<String,String>{
  // RESET AND ASSIGN ID
  let _ = option.reset();

  // VALIDATE SELECT ID 
  if let Some(select_id) = &option.select_id{
    let select_requested = get_by_id::<Select>(&client,"crabs_test","selects",select_id.as_str()).await.expect("Failed on db level");
    if select_requested == None{
      return Err("Select with the given id doesn't exist".to_string())
    }
  }

  // ID OF CREATED OPTION
  let option_id = insert_doc(&client, "crabs_test", "options", &option).await.unwrap().inserted_id.to_string();

  // UPDATE OPTIONS IN THE SELECT
  if let Some(select_id) = &option.select_id{
    let document = doc! { "$push": { "options": &option_id.trim_matches('"').to_string() } };
    update_push::<Select>(&client, "crabs_test", "selects", document, select_id).await;
  }

  Ok(option_id.trim_matches('"').to_string())

}