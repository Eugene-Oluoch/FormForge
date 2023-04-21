use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson,Document};
use crate::{models::{
  traits::{ResetDefaults},
  input::{Input},
  select::{SelectReceive}
}, utils::generate_current_time};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Form {
  pub _id: Option<String>,
  pub name:Option<String>,
  pub inputs: Option<Vec<String>>,
  pub selects: Option<Vec<String>>,
  pub steps:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FormReceive {
  pub _id: Option<String>,
  pub name:Option<String>,
  pub inputs: Option<Vec<Input>>,
  pub selects: Option<Vec<SelectReceive>>,
  pub steps:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>
}

impl Form{
  pub fn new() -> Self{
    Self {
        created_at: None,
        updated_at: None,
        archive:None,
        inputs:Some(vec![]),
        steps: None,
        name: Some(String::from("default")),
        selects: Some(vec![]),
        _id:None
    }
  }

}

impl FormReceive {
  pub fn convert(&self,inputs:Option<Vec<String>>,selects:Option<Vec<String>>) -> Form{
    Form { 
      _id: None, 
      name: self.name.clone(), 
      inputs, 
      selects, 
      steps: self.steps, 
      archive: None, 
      updated_at: None, 
      created_at: None }
  }
}

impl ResetDefaults for Form{
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


impl From<Form> for Bson {
  fn from(option: Form) -> Self {
    let mut doc = Document::new();
    doc.insert("name", option.name);
    doc.insert("inputs", option.inputs);
    doc.insert("selects", option.selects);

    if let Some(steps) = option.steps {
        doc.insert("steps", steps);
    }

    if let Some(id) = option._id {
      doc.insert("_id", id);
    }
    
    Bson::Document(doc)
  }
}
