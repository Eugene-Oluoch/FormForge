use super::input::Input;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CheckRadio{
  name:String,
  inputs:Vec<Input>
}

impl CheckRadio {
  pub fn new(name:String,inputs:Vec<Input>) ->Self{
    Self { name, inputs}
  }
}