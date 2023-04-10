use crate::{utils::string_to_object_id};
use mongodb::bson::{oid::ObjectId, Bson, Document};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Select{
  form_id:Option<ObjectId>,
  multiple:bool,
  size:Option<String>,
  options:Vec<ObjectId>,
  validation:Option<String>,
  step:Option<i32>,
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  id: Option<ObjectId>
}

impl Select {
  pub fn new() -> Self{
    Self {
      multiple: false,
      size:None,
      options: vec![],
      validation:None,
      step:None,
      id:None,
      form_id:None
    }
  }


  // Setters

  pub fn set_multiple(&mut self,multiple:&bool)-> &mut Self{
    self.multiple = *multiple;
    self
  }

  pub fn set_size(&mut self,size:&str) -> &mut Self{
    self.size = Some(size.to_string());
    self
  }

  pub fn set_form_id(&mut self,id:&str) -> &mut Self{
    self.form_id = Some(string_to_object_id(id));
    self
  }

  pub fn set_step(&mut self,step:&i32) -> &mut Self{
    self.step = Some(*step);
    self
  }

  pub fn build(self) -> Self{self}

  pub fn add_option(&mut self, id:&str) -> &mut Self{
    self.options.push(string_to_object_id(id));
    self
  }

  // Getters
  pub fn get_id(&self) -> Option<ObjectId>{
    self.id
  }


  pub fn get_options(&self)->Vec<String>{
    let mut options = Vec::new();
    for option in &self.options{
      options.push(option.to_hex());
    }
    options
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

    if let Some(id) = option.id {
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