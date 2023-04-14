use serde::{Serialize, Deserialize};
use mongodb::{bson::{Document, doc, Bson}};


// ADD ARCHIVE TO HANDLE DELETE -> SOFT DELETE
#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct OptionSelect{
  pub _id: Option<String>,
  pub name:Option<String>,
  pub selected:Option<bool>,
  pub value:Option<String>,
  pub select_id:Option<String>,
  archive:Option<bool>,
  updated_at: Option<i64>,
  created_at: Option<i64>
}


impl OptionSelect {
  pub fn new () -> Self{
    Self {
      created_at: None,
      updated_at: None,
      archive:None,
      select_id:None,
      selected:Some(false),
      name:None,
      value:None,
      _id:None
    }
  }

  // Setters
  pub fn set_name(&mut self,value:Option<String>) -> &mut Self{
    if let Some(value) = value{
      self.name = Some(value.to_string());
    }
    self
  }

  pub fn set_value(&mut self,value:Option<String>) -> &mut Self{
    if let Some(value) = value{
      self.value = Some(value.to_string());
    }
    self
  }

  pub fn set_selected(&mut self,value:Option<bool>) -> &mut Self{
    if let Some(value) = value{
      self.selected = Some(value);
    }
    self
  }

  pub fn set_select_id(&mut self,id:Option<String>)-> &mut Self{
    if let Some(id) = id{
      self.select_id = Some(id);
    }
    self
  }
  pub fn set_id(&mut self,id:Option<String>) -> &mut Self{
    if let Some(id) = id{
      self._id = Some(id);
    }else{
      self._id = None
    }
    self
  }

  pub fn build(&self){

  }

  pub fn to_document(&self) -> Document {
    doc! {
        "selected": self.selected,
        "name": self.name.as_deref().unwrap_or("").to_string(),
        "value": self.value.as_deref().unwrap_or("").to_string(),
        "_id": self._id.clone().unwrap_or_else(|| String::new()),
    }
}




  // Getters
  pub fn get_id(&self) -> &Option<String>{
    &self._id
  }

  pub fn get_select_id(&self) -> &Option<String>{
    &self.select_id
  }

}

impl From<OptionSelect> for Bson {
  fn from(option: OptionSelect) -> Self {
      let mut doc = Document::new();
      doc.insert("selected", option.selected);
      if let Some(name) = option.name {
          doc.insert("name", name);
      }
      if let Some(value) = option.value {
          doc.insert("value", value);
      }
      if let Some(id) = option._id {
          doc.insert("_id", id);
      }
      Bson::Document(doc)
  }

}
