use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Week{
  max:String,
  min:String,
  steps:String
}


impl Week {
  pub fn new() -> Self{
    Self {
      min:"10".to_string(),
      max:"20".to_string(),
      steps:"hello".to_string()
    }
  }
}