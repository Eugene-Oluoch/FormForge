use serde::{Serialize, Deserialize};
use uuid::Uuid;
use crate::utils::{generate_current_time};
use super::traits::ResetDefaults;

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct TextArea{
  pub form_id:Option<String>,
  pub rows:Option<String>,
  pub cols:Option<String>,
  pub _id:Option<String>,
  pub created_at:Option<i64>,
  pub updated_at:Option<i64>,
  pub archive:Option<bool>,
  pub required:Option<bool>
}


impl ResetDefaults for TextArea{
  fn reset(&mut self) {
    self.updated_at = Some(generate_current_time());
    self.created_at = Some(generate_current_time());
    self.archive = Some(false);
    self._id = Some(Uuid::new_v4().to_string());
    if self.required.is_none(){
      self.required = Some(false)
    }

    if self.cols.is_none(){
      self.cols = Some("33".to_string())
    }

    if self.rows.is_none(){
      self.rows = Some("5".to_string())
    }

  }
  fn update(&mut self) {
      self.updated_at = Some(generate_current_time())
  }
}