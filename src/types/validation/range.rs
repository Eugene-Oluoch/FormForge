use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Range{
  min:String,
  max:String,
  steps:String
}


impl Range {
  pub fn new() -> Self{
    Self {
      min:"10".to_string(),
      max:"20".to_string(),
      steps:"hello".to_string()
    }
  }
}