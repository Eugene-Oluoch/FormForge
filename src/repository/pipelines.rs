use mongodb::bson::{doc, Document};

pub fn remove_option_from_selects(id:&str) -> Vec<Document>{
  vec![
    doc! {
      "options":id
    },
    doc! {
    "$pullAll":{
      "options":[id]
    }
    }
  ]
}