use crate::validation::{Validate};
use crate::models::traits::ResetDefaults;
use serde::{Serialize, Deserialize};
use mongodb::bson::{Bson, Document};
use uuid::Uuid;


#[derive(Serialize, Deserialize, Debug)]
pub struct Input{
  pub _id: Option<String>,
  pub form_id:Option<String>,
  pub type_identifier:String,
  pub disabled:bool,
  pub placeholder:Option<String>,
  pub label:Option<String>,
  pub name:String,
  pub validation:Option<Validate>,
  pub step:Option<i32>,
  pub archive:Option<bool>,
  pub updated_at: Option<i64>,
  pub created_at: Option<i64>,
}



impl Input  {
  pub fn new() -> Self{
    Self {
      created_at: None,
      updated_at: None,
      archive:None,
      form_id:None,
      type_identifier:"text".to_string(),
      placeholder:None,
      label:None,
      disabled: false,
      name:String::from("name"),
      validation:Some(Validate::new()),
      step:None,
      _id:None
    }
  }
  pub fn map_type(&mut self){
    let types = vec![
      "color",
      "date",
      "email",
      "number",
      "password",
      "range",
      "tel",
      "text",
      "time",
      "url",
      "week",
      "file",
      "month",
      "datetime-local"
    ];

    if types.contains(&self.type_identifier.to_lowercase().as_str()) == false{
      self.type_identifier = "text".to_string();
    }
  }

}

impl ResetDefaults for Input{
  fn reset(&mut self) {
      self.archive = None;
      self.created_at = None;
      self.updated_at = None;
      self._id = Some(Uuid::new_v4().to_string());
  }
}

// This ain't Complete
impl From<Input> for Bson {
  fn from(option: Input) -> Self {
      let mut doc = Document::new();
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