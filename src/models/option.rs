use serde::{Serialize, Deserialize};
use mongodb::{bson::{Document, doc, Bson}};
use crate::models::traits::ResetDefaults;
use uuid::Uuid;

// ADD ARCHIVE TO HANDLE DELETE -> SOFT DELETE
#[derive(Serialize, Deserialize, Debug,PartialEq)]
pub struct OptionSelect{
  pub _id: Option<String>,
  pub name:Option<String>,
  pub selected:Option<bool>,
  pub value:Option<String>,
  pub select_id:Option<String>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>
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

  pub fn to_document(&self) -> Document {
    doc! {
        "selected": self.selected,
        "name": self.name.as_deref().unwrap_or("").to_string(),
        "value": self.value.as_deref().unwrap_or("").to_string(),
        "_id": self._id.clone().unwrap_or_else(|| String::new()),
    }
}

}

impl ResetDefaults for OptionSelect{
  fn reset(&mut self) {
      self.updated_at = None;
      self.created_at = None;
      self.archive = None;
      self._id = Some(Uuid::new_v4().to_string())
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
