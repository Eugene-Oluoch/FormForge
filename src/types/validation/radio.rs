// Name in this scope represents the group name for radio input fields

use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Radio{
  name:String,
  checked:bool
}


impl Radio {
  pub fn new() -> Self{
    Self {
      name:"default".to_string(),
      checked:true
    }
  }
}