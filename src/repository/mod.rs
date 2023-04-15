pub mod pipelines;
use std::collections::HashMap;
use mongodb::{bson::{ doc, Document}};

pub fn map(name:&str, id:&str) -> Vec<Document>{
  // RETRIEVES A PIPELINE BASED IN THE NAME SUPPLIED
  pipelines(id).get(name).unwrap().to_vec()
}

fn pipelines (id:&str) -> HashMap<&str,Vec<Document>>{

  // FORM PIPELINE FOR FETCHING FORM AND ALL REFERENCE RELATIONSHIP
  let form_pipeline = vec![
    doc! {
      "$match": {
          "_id": id
      }
    },
    doc! {
      "$lookup": {
        "from": "inputs",
        "localField": "inputs",
        "foreignField": "_id",
        "as": "inputs"
      }
    },
    doc! {
      "$lookup": {
      "from": "selects",
      "localField": "selects",
      "foreignField": "_id",
      "as": "selects"
    }
    },
    doc! {
      "$unwind": {
          "path": "$selects",
          "preserveNullAndEmptyArrays": true
      }
  },
  doc! {
      "$match": {
          "selects": { "$ne": [] }
      }
  },
  doc! {
      "$lookup": {
          "from": "options",
          "localField": "selects.options",
          "foreignField": "_id",
          "as": "selects.options"
      }
  },
  doc! {
      "$group": {
          "_id": "$_id",
          "name": { "$first": "$name" },
          "steps": { "$first": "$steps" },
          "archive": { "$first": "$archive" },
          "updated_at": { "$first": "$updated_at" },
          "created_at": { "$first": "$created_at" },
          "inputs": {
              "$first": "$inputs"
          },
          "selects": {
              "$push": "$selects"
          }
      }
  }
];
  // SELECT PIPELINE FOR FETCHING FORM AND ALL REFERENCE RELATIONSHIP
  let select_pipeline = vec![
    doc! {
      "$match": {
        "_id": id
    }
    },
    doc! {
        "$lookup": {
        "from": "options",
        "localField": "options",
        "foreignField": "_id",
        "as": "options"
      }
    },
  ];

  let mut pipeline_value = HashMap::new();
  let _ = &mut pipeline_value.insert("select", select_pipeline);
  let _ = &mut pipeline_value.insert("form", form_pipeline);
  pipeline_value
}
