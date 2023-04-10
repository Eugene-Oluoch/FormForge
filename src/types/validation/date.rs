use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Date{
  value:String,
  min:String,
  max:String,
  steps:String
}

impl Date {
  pub fn new() -> Self{
    Self {
      value:"default".to_string(),
      min:"10".to_string(),
      max:"20".to_string(),
      steps:"hello".to_string()
    }
  }
}