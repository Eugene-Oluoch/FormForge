use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Url{
  maxlength:String,
  minlength:String,
  pattern:String
}

impl Url {
  pub fn new() -> Self{
    Self {
      minlength:"10".to_string(),
      maxlength:"20".to_string(),
      pattern:"hello".to_string()
    }
  }
}