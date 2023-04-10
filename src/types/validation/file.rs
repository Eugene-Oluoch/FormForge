use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct File{
  accept:String,
  capture:String,
  multiple:bool
}

impl File {
  pub fn new() -> Self{
    Self {
      accept:"default".to_string(),
      capture:"10".to_string(),
      multiple:false
    }
  }
}