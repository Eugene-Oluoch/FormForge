use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Password{
  maxlength:String,
  minlength:String,
  autocomplete:String,
  inputmode:String,
}

impl Password {
  pub fn new() -> Self{
    Self {
      autocomplete:"default".to_string(),
      minlength:"10".to_string(),
      maxlength:"20".to_string(),
      inputmode:"hello".to_string()
    }
  }
}