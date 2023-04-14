use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson,Document};


#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Form {
  _id: Option<String>,
  name:String,
  inputs: Vec<String>,
  selects: Vec<String>,
  steps:Option<i32>,
  archive:Option<bool>,
  updated_at: Option<i64>,
  created_at: Option<i64>
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

  // Setters
  pub fn set_steps(&mut self,number:&i32) -> & mut Self{
    self.steps = Some(*number);
    self
  }

  pub fn set_name(&mut self,name:String) -> & mut Self{
    self.name = name;
    self
  }

  pub fn set_id(&mut self, id:String) -> &mut Self{
    self._id = Some(id.to_string());
    self
  }

  pub fn build(&self){}

  pub fn add_input(&mut self,id:&str) -> &mut Self{
    self.inputs.push(id.to_string());
    self
  }

  pub fn add_select(&mut self,id:&str) -> &mut Self{
    self.selects.push(id.to_string());
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

    if let Some(id) = option._id {
      doc.insert("_id", id);
    }
    
    Bson::Document(doc)
  }
}
