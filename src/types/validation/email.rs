// If validate is true as the user to supply a regex or use the default one
use serde::{Serialize, Deserialize};
use regex::Regex;


#[derive(Serialize, Deserialize, Debug)]
pub struct Email{
  validate:bool,
  multiple:bool
}

impl Email{
  pub fn new() -> Self{
    Self {
      validate:true,
      multiple:false
    }
  }

  // Setters
  pub fn set_validate(&mut self,value:bool){
    self.validate = value;
  }

  //Getters
  pub fn get_validate(&self) -> bool{
    self.validate
  }

  // Check if email is valid
  pub fn validate(value:&str) -> bool{
    let email_regex = Regex::new(r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})").unwrap();
    email_regex.is_match(value)
  }

}
