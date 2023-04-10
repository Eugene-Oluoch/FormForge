use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct DateTimeLocal{
  value:String,
  min:String,
  max:String,
  step:String
}

impl DateTimeLocal {
  pub fn new() -> Self{
    Self {
      value:"default".to_string(),
      min:"10".to_string(),
      max:"20".to_string(),
      step:"hello".to_string()
    }
  }
}