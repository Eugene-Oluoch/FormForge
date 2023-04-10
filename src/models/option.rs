use serde::{Serialize, Deserialize};
use mongodb::{bson::{oid::ObjectId, Document, doc, Bson}};
use crate::utils::string_to_object_id;


#[derive(Serialize, Deserialize, Debug)]
pub struct OptionSelect{
  pub select_id:Option<ObjectId>,
  pub selected:Option<bool>,
  pub name:Option<String>,
  pub value:Option<String>,
  #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
  pub id: Option<ObjectId>
}

#[derive(Serialize,Deserialize,Debug)]
pub struct OptionSelectReceive{
  pub select_id:Option<String>,
  pub selected:Option<bool>,
  pub name:Option<String>,
  pub value:Option<String>,
  pub id: Option<String>
}

impl OptionSelectReceive{

  pub fn new() -> Self{
    Self {
      select_id:None,
      selected:Some(false),
      name:None,
      value:None,
      id:None
    }
  }

  pub fn set_selected(&mut self,value:Option<bool>) -> &mut Self{
    if let Some(value) = value{
      self.selected = Some(value);
    }
    self
  }

  pub fn set_name(&mut self,value:Option<String>) -> &mut Self{
    if let Some(value) = value{
      self.name = Some(value.to_string());
    }
    self
  }

  pub fn set_value(&mut self, value:Option<String>) -> &mut Self{
    if let Some(value) = value{
      self.value = Some(value.to_string());
    }
    self
  }

  pub fn set_select_id(&mut self, value:Option<ObjectId>) -> &mut Self{
    if let Some(value) = value{
      self.select_id = Some(value.to_hex());
    }
    self
  }

  pub fn set_id(&mut self, value:Option<ObjectId>) -> &mut Self{
    if let Some(value) = value{
      self.id = Some(value.to_string());
    }
    self
  }

  pub fn build(&self){}

  pub fn convert(self) -> OptionSelect{
    let mut option = OptionSelect::new();
    let _ = &option
      .set_name(self.name)
      .set_selected(self.selected)
      .set_value(self.value)
      .set_select_id(self.select_id)
      .set_id(self.id)
      .build();
    option
  }
}


impl OptionSelect {
  pub fn new () -> Self{
    Self {
      select_id:None,
      selected:Some(false),
      name:None,
      value:None,
      id:None
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
      self.select_id = Some(string_to_object_id(id.as_str()));
    }
    self
  }
  pub fn set_id(&mut self,id:Option<String>) -> &mut Self{
    if let Some(id) = id{
      self.id = Some(string_to_object_id(id.as_str()));
    }else{
      self.id = None
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
        "_id": self.id.clone().unwrap_or_else(|| ObjectId::new()),
    }
}

  pub fn convert(self) -> OptionSelectReceive{
    let mut option = OptionSelectReceive::new();
    let _ = &option
      .set_name(self.name)
      .set_selected(self.selected)
      .set_value(self.value)
      .set_select_id(self.select_id)
      .set_id(self.id)
      .build();
    option 
  }


  // Getters
  pub fn get_id(&self) -> Option<ObjectId>{
    self.id
  }

  pub fn get_select_id(&self) -> Option<ObjectId>{
    self.select_id
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
      if let Some(id) = option.id {
          doc.insert("_id", id);
      }
      Bson::Document(doc)
  }

}
