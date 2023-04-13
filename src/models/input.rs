use crate::types::{self};
use types::Types;
use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson, Document};

#[derive(Serialize, Deserialize, Debug)]
pub struct Input{
  created_at: Option<i64>,
  updated_at: Option<i64>,
  archive:Option<bool>,
  form_id:Option<String>,
  type_identifier:types::Types,
  disabled:bool,
  placeholder:Option<String>,
  label:Option<String>,
  required:bool,
  name:String,
  pub validation:Option<String>,
  step:Option<i32>,
  _id: Option<String>
}


impl Input  {
  pub fn new() -> Self{
    Self {
      created_at: None,
      updated_at: None,
      archive:None,
      form_id:None,
      type_identifier:types::Types::Text,
      placeholder:None,
      label:None,
      required:false,
      disabled: false,
      name:String::from("name"),
      validation:Some(String::from("To hold for now")),
      step:None,
      _id:None
    }
  }

  // Setters
  pub fn set_type(&mut self,option:&str) -> &mut Self{
    self.type_identifier = Types::map(option).unwrap();
    // self.validation = Validation::map(option).unwrap();
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
  pub fn get_id(&self) -> &Option<String>{
    &self._id
  }



}

// This ain't Complete
impl From<Input> for Bson {
  fn from(option: Input) -> Self {
      let mut doc = Document::new();
      doc.insert("required", option.required);
      doc.insert("disabled", option.disabled);
      doc.insert("step", option.step);

      // if option.type_identifier == types::Types::Color{
      //   doc.insert("validation", Validation::into_color(option.validation));
      // }
    

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

    if let Some(id) = option._id {
      doc.insert("_id", id);
    }
    
    Bson::Document(doc)
  }
}