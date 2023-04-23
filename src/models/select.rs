use mongodb::bson::{Bson, Document};
use serde::{Serialize, Deserialize};
use crate::OptionSelect;
use crate::models::traits::ResetDefaults;
use uuid::Uuid;
use crate::utils::{generate_current_time};

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Select{
  pub _id: Option<String>,
  pub form_id:Option<String>,
  pub multiple:Option<bool>,
  pub size:Option<String>,
  pub options:Vec<String>,
  pub label:Option<String>,
  pub required:Option<bool>,
  pub step:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct SelectReceive{
  pub _id: Option<String>,
  pub form_id:Option<String>,
  pub multiple:Option<bool>,
  pub size:Option<String>,
  pub options:Option<Vec<OptionSelect>>,
  pub label:Option<String>,
  pub required:Option<bool>,
  pub step:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>,
}

impl ResetDefaults for SelectReceive{
  fn reset(&mut self) {
    self.updated_at = Some(generate_current_time());
    self.created_at = Some(generate_current_time());
    self.archive = Some(false);
    self._id = Some(Uuid::new_v4().to_string());
    self.required = Some(false);
  }
  fn update(&mut self) {
      self.updated_at = Some(generate_current_time())
  }
}

impl ResetDefaults for Select{
  fn reset(&mut self) {
    self.updated_at = Some(generate_current_time());
    self.created_at = Some(generate_current_time());
    self.archive = Some(false);
    self._id = Some(Uuid::new_v4().to_string())
  }
  fn update(&mut self) {
      self.updated_at = Some(generate_current_time())
  }
}

impl SelectReceive{
  pub fn convert(&self) -> Select {
    Select { 
      _id: self._id.clone(), 
      form_id: self.form_id.clone(), 
      multiple: self.multiple.clone(), 
      size: self.size.clone(), 
      options: Vec::new(),
      label:None,
      required: self.required.clone(), 
      step: self.step.clone(), 
      archive: self.archive.clone(), 
      updated_at: self.updated_at.clone(), 
      created_at: self.created_at.clone() 
    }
  }
}


impl Select {
  pub fn new() -> Self{
    Self {
      created_at: None,
      updated_at: None,
      archive:None,
      multiple: None,
      size:None,
      options: vec![],
      label:None,
      required:None,
      step:None,
      _id:None,
      form_id:None
    }
  }
}


impl From<Select> for Bson {
  fn from(option: Select) -> Self {
    let mut doc = Document::new();
    doc.insert("multiple", option.multiple);
    doc.insert("options", option.options);

    if let Some(step) = option.step {
        doc.insert("step", step);
    }

    if let Some(id) = option._id {
      doc.insert("_id", id);
    }
    
    if let Some(form_id) = option.form_id {
      doc.insert("form_id", form_id);
    }

    if let Some(size) = option.size{
      doc.insert("size", size);
    }

    if let Some(req) = option.required{
      doc.insert("required",req);
    }

    Bson::Document(doc)
  }
}