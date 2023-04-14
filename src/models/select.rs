use mongodb::bson::{Bson, Document};
use serde::{Serialize, Deserialize};
use crate::OptionSelect;

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct Select{
  _id: Option<String>,
  form_id:Option<String>,
  multiple:bool,
  size:Option<String>,
  options:Vec<String>,
  validation:Option<String>,
  step:Option<i32>,
  archive:Option<bool>,
  updated_at: Option<i64>,
  created_at: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct SelectReceive{
  created_at: Option<i64>,
  updated_at: Option<i64>,
  archive:Option<bool>,
  form_id:Option<String>,
  multiple:bool,
  size:Option<String>,
  options:Vec<OptionSelect>,
  validation:Option<String>,
  step:Option<i32>,
  _id: Option<String>
}

impl SelectReceive {
    // SETTERS
    pub fn set_id(&mut self,id:String)-> &mut Self{
      self._id = Some(id);
      self
    }
    // GETTERS
    pub fn get_form_id(&self) -> &Option<String>{
      &self.form_id
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
    self.form_id = Some(id.to_string());
    self
  }

  pub fn set_step(&mut self,step:&i32) -> &mut Self{
    self.step = Some(*step);
    self
  }

  pub fn build(self) -> Self{self}

  pub fn add_option(&mut self, id:&str) -> &mut Self{
    self.options.push(id.to_string());
    self
  }

  // Getters
  pub fn get_id(&self) -> &Option<String>{
    &self._id
  }


  pub fn get_options(&self)->&Vec<String>{
    &self.options
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