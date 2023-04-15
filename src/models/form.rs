use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson,Document};
use crate::models::{
  traits::{ResetDefaults},
  input::{Input},
  select::{SelectReceive}
};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Form {
  pub _id: Option<String>,
  pub name:String,
  pub inputs: Vec<String>,
  pub selects: Vec<String>,
  pub steps:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FormReceive {
  pub _id: Option<String>,
  pub name:String,
  pub inputs: Vec<Input>,
  pub selects: Vec<SelectReceive>,
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
        inputs: vec![],
        steps: None,
        name: String::from("default"),
        selects: vec![],
        _id:None
    }
  }

}

impl ResetDefaults for Form{
  fn reset(&mut self) {
    self.updated_at = None;
    self.created_at = None;
    self.archive = None;
    self._id = Some(Uuid::new_v4().to_string())
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
