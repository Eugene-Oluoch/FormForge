use crate::utils::string_to_object_id;
use rocket::request::FromParam;
use serde::{Serialize, Deserialize};
use mongodb::bson::Bson;
use mongodb::bson::Document;
use mongodb::bson::oid::ObjectId;

#[derive(Serialize, Deserialize, Debug)]
pub struct Form {
  inputs: Vec<ObjectId>,
  selects: Vec<ObjectId>,
  steps:Option<i32>,
  name:String,
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  id: Option<ObjectId>
}


impl Form{
  pub fn new() -> Self{
    Self {
        inputs: vec![],
        steps: None,
        name: String::from("default"),
        selects: vec![],
        id:None
    }
  }

  // Setters
  pub fn set_steps(&mut self,number:&i32) -> & mut Self{
    self.steps = Some(*number);
    self
  }

  pub fn set_name(&mut self,name:String) -> & mut Self{
    self.name = name;
    self
  }

  pub fn build(&self){}

  pub fn add_input(&mut self,id:&str) -> &mut Self{
    self.inputs.push(string_to_object_id(&id));
    self
  }

  pub fn add_select(&mut self,id:&str) -> &mut Self{
    self.selects.push(string_to_object_id(&id));
    self
  }

  // Getters
  pub fn get_name(&self)-> &String{
    &self.name
  }

  pub fn get_steps(&self) -> i32{
    self.steps.unwrap()
  }


  // pub fn get_select(&self, id:Option<ObjectId>) -> &Select{
  //   self.selects
  //     .iter()
  //     .find(| x | x.get_id() == id).unwrap()
  // }

  // pub fn get_input(&self, id:Option<ObjectId>) -> &Input{
  //   self.inputs
  //     .iter()
  //     .find(| x |  x.get_id() == id)
  //     .unwrap()
  // }


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

    if let Some(id) = option.id {
      doc.insert("_id", id);
    }
    
    Bson::Document(doc)
  }
}
