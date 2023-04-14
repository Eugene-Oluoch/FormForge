use serde::{Serialize, Deserialize};

/* 
Chechbox and Radio to be handled differently
*/
#[derive(Serialize, Deserialize, Debug)]
pub struct Validate{
  #[serde(skip_serializing_if = "Option::is_none")]
  pub min:Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max:Option<i32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub maxlength:Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub minlength:Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub required:Option<bool>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub pattern:Option<String>,
}


impl Validate{
  pub fn new() -> Self{
    Self {
      min:None,
      max:None,
      maxlength:None,
      minlength:None,
      required:Some(false),
      pattern:None,
    }
  }

  pub fn filter_validation(&mut self,match_type:&str) -> &Self{
    match match_type {
      "number" | "range" => {
        self.maxlength = None;
        self.minlength = None;
        self.pattern = None;
        self
      },
      "url" | "password" | "text" | "email" | "tel" => {
        self.minlength = None;
        self.maxlength = None;
        self
      },
      "month" | "time" | "week" | "date" | "datelocal-time" => {
        self.min = None;
        self.max = None;
        self.pattern = None;
        self
      },
      _=>todo!()
    }
  }

}
