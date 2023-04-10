use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Month{
  min:String,
  max:String,
}

impl Month {
  pub fn new() -> Self{
    Self {
      min:"10".to_string(),
      max:"20".to_string()
    }
  }
}