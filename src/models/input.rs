use crate::types::{self, validation::color::Color};
use types::validation::Validation;
use types::Types;
use serde::{Serialize, Deserialize};
use mongodb::bson::{oid::ObjectId, Bson, Document};
use types::validation::color;

#[derive(Serialize, Deserialize, Debug)]
pub struct Input{
  form_id:Option<ObjectId>,
  type_identifier:types::Types,
  disabled:bool,
  placeholder:Option<String>,
  label:Option<String>,
  required:bool,
  name:String,
  pub validation:Validation,
  step:Option<i32>,
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  id: Option<ObjectId>
}


impl Input  {
  pub fn new() -> Self{
    Self {
      form_id:None,
      type_identifier:types::Types::Text,
      placeholder:None,
      label:None,
      required:false,
      disabled: false,
      name:String::from("name"),
      validation:Validation::Color(Color::new()),
      step:None,
      id:None
    }
  }

  // Setters
  pub fn set_type(&mut self,option:&str) -> &mut Self{
    self.type_identifier = Types::map(option).unwrap();
    self.validation = Validation::map(option).unwrap();
    self
  }

  pub fn set_step(&mut self,value:i32) -> &mut Self{
    self.step = Some(value);
    self
  }

  pub fn set_name(&mut self,value:String) -> & mut Self{
    self.name = value;
    self
  }

  pub fn set_label(& mut self, value:String) -> & mut Self{
    self.label = Some(value);
    self
  }

  pub fn set_placeholder(&mut self,value:String) -> &mut Self{
    self.placeholder = Some(value);
    self
  }

  pub fn set_disabled(&mut self,value:bool) -> &mut Self{
    self.disabled = value;
    self
  }


  pub fn set_required(&mut self, value:bool) -> &mut Self{
    self.required = value;
    self
  }

  pub fn build(&self){}

  // Getters
  pub fn get_id(&self) -> Option<ObjectId>{
    self.id
  }



}

// This ain't Complete
impl From<Input> for Bson {
  fn from(option: Input) -> Self {
      let mut doc = Document::new();
      doc.insert("required", option.required);
      doc.insert("disabled", option.disabled);
      doc.insert("step", option.step);

      if option.type_identifier == types::Types::Color{
        doc.insert("validation", Validation::into_color(option.validation));
      }
    

      if let Some(placeholder) = option.placeholder {
          doc.insert("placeholder", placeholder);
      }

      if let Some(label) = option.label {
        doc.insert("label", label);
      } 

      if let Some(form_id) = option.form_id {
        doc.insert("form_id", form_id);
      }

      if let Some(step) = option.step {
        doc.insert("step", step);
    }

    if let Some(id) = option.id {
      doc.insert("_id", id);
    }
    
    Bson::Document(doc)
  }
}