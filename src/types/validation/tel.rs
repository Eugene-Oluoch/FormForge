use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Tel{
  maxlength:String,
  minlength:String,
  pattern:String
}

impl Tel {
  pub fn new() -> Self{
    Self {
      minlength:"10".to_string(),
      maxlength:"20".to_string(),
      pattern:"hello".to_string()
    }
  }
}