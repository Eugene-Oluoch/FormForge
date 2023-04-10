use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Text{
  pub minlength:Option<i32>,
  pub maxlength:Option<i32>
}

impl Text{
  pub fn new() -> Self{
    Self {
      minlength:None,
      maxlength:None
    }
  }

  // Setters
  pub fn set_min(&mut self,value:i32) -> &mut Self{
    self.minlength = Some(value);
    self
  }

  pub fn set_max(&mut self,value:i32) -> &mut Self{
    self.maxlength = Some(value);
    self
  }

  // Getter
  pub fn get_min(&self) -> i32{
    self.minlength.unwrap()
  }

  pub fn get_max(&self) -> i32{
    self.maxlength.unwrap()
  }


}