use mongodb::{bson::{Bson, doc, Document}, error::Result};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Color{
  value:String
}


impl Color{
  pub fn new() -> Self{
    Self {
      value: "#FFFFFF".to_string()
    }
  }
}

impl From<Color> for Bson {
  fn from(option: Color) -> Self {
      let mut doc = Document::new();
      doc.insert("value",option.value);
    
    Bson::Document(doc)
  }
}
