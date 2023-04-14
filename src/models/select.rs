use mongodb::bson::{Bson, Document};
use serde::{Serialize, Deserialize};
use crate::OptionSelect;
use crate::models::traits::ResetDefaults;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Select{
  pub _id: Option<String>,
  pub form_id:Option<String>,
  pub multiple:bool,
  pub size:Option<String>,
  pub options:Vec<String>,
  pub validation:Option<String>,
  pub step:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct SelectReceive{
  pub _id: Option<String>,
  pub form_id:Option<String>,
  pub multiple:bool,
  pub size:Option<String>,
  pub options:Vec<OptionSelect>,
  pub validation:Option<String>,
  pub step:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>,
}

impl ResetDefaults for SelectReceive{
  fn reset(&mut self) {
    self.updated_at = None;
    self.created_at = None;
    self.archive = None;
    self._id = Some(Uuid::new_v4().to_string())
  }
}


impl Select {
  pub fn new() -> Self{
    Self {
      created_at: None,
      updated_at: None,
      archive:None,
      multiple: false,
      size:None,
      options: vec![],
      validation:None,
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

    if let Some(validation) = option.validation{
      doc.insert("validation",validation);
    }

    Bson::Document(doc)
  }
}