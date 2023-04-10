use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Time{
  max:String,
  min:String,
  step:String
}

impl Time {
  pub fn new() -> Self{
    Self {
      min:"10".to_string(),
      max:"20".to_string(),
      step:"hello".to_string()
    }
  }
}