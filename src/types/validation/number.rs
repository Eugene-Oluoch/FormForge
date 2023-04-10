use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Number{
  min:String,
  max:String,
  step:String
}

impl Number {
  pub fn new() -> Self{
    Self {
      min:"10".to_string(),
      max:"20".to_string(),
      step:"hello".to_string()
    }
  }
}