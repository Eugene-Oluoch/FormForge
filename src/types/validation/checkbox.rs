use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct CheckBox{
  name:String,
  value:String,
  checked:bool,
}

impl CheckBox{
  pub fn new()-> Self{
    Self {
      name:"default".to_string(),
      value:"default".to_string(),
      checked:true
    }
  }
}