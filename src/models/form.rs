use std::{collections::HashMap, cell::RefCell, sync::Arc};

use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson,Document};
use tokio::sync::Mutex;
use crate::{models::{
  traits::{ResetDefaults},
  input::{Input},
  select::{SelectReceive},
  check_radio::{CheckRadio}
}, utils::generate_current_time};
use uuid::Uuid;

use super::textarea::TextArea;

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Form {
  pub _id: Option<String>,
  pub name:Option<String>,
  pub inputs: Option<Vec<String>>,
  pub selects: Option<Vec<String>>,
  pub steps:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>,
  pub textareas:Option<Vec<String>>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct FormReceive {
  pub _id: Option<String>,
  pub name:Option<String>,
  pub inputs: Option<Vec<Input>>,
  pub textareas:Option<Vec<TextArea>>,
  pub selects: Option<Vec<SelectReceive>>,
  pub steps:Option<i32>,
  pub checkboxes:Option<Vec<Input>>,
  pub radios:Option<Vec<Input>>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FormReceiveFinal {
  pub _id: Option<String>,
  pub name:Option<String>,
  pub inputs: Option<Vec<Input>>,
  pub textareas:Option<Vec<TextArea>>,
  pub selects: Option<Vec<SelectReceive>>,
  pub steps:Option<i32>,
  pub checkboxes:Vec<CheckRadio>,
  pub radios:Vec<CheckRadio>,
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
        textareas:Some(vec![]),
        steps: None,
        name: Some(String::from("default")),
        selects: Some(vec![]),
        _id:None
    }
  }

}

impl FormReceive {
  pub fn convert(&self,inputs:Option<Vec<String>>,selects:Option<Vec<String>>,textareas:Option<Vec<String>>) -> Form{
    Form { 
      _id: None, 
      name: self.name.clone(), 
      inputs, 
      selects,
      textareas,
      steps: self.steps, 
      archive: None, 
      updated_at: None, 
      created_at: None }
  }
  pub async fn to_final(self,inputs:Option<Vec<Input>>,checkboxes:HashMap<String, Vec<Input>>,radios:HashMap<String, Vec<Input>>) -> FormReceiveFinal{
    let mut return_checkboxes = Vec::new();
    let mut return_radios = Vec::new();

    // THREADS TO STRUCTURE THE DATA
    let checkboxes_thread = tokio::spawn(async move{
      for (k,v) in checkboxes{
        return_checkboxes.push(CheckRadio::new(k, v));
      }
      return_checkboxes
    });
  
    let radios_thread = tokio::spawn(async move{
      for (k,v) in radios{
        return_radios.push(CheckRadio::new(k, v));
      }
      return_radios
    });

    let results = tokio::join!(checkboxes_thread,radios_thread);

    FormReceiveFinal { 
      _id: self._id, 
      name: self.name, 
      inputs, 
      selects:self.selects,
      textareas:self.textareas,
      checkboxes:results.0.expect("failed"),
      radios:results.1.expect("failed"),
      steps: self.steps, 
      archive: self.archive, 
      updated_at: self.updated_at, 
      created_at: self.created_at } 

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
