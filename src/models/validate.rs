use std::collections::HashMap;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Validate{
  pub form_id:Option<String>,
  pub inputs:Option<HashMap<String,String>>,
  pub selects:Option<HashMap<String,String>>
}