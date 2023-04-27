use std::collections::HashMap;
use mongodb::{bson::{ doc, Document}};

pub fn map(name:&str, id:&str) -> Vec<Document>{
  // RETRIEVES A PIPELINE BASED IN THE NAME SUPPLIED
  pipelines(id).get(name).unwrap().to_vec()
}

fn pipelines (id:&str) -> HashMap<&str,Vec<Document>>{

  // FORM PIPELINE FOR FETCHING FORM AND ALL REFERENCE RELATIONSHIP
  // TODO FIX PIPELINE NOT RETURNING SELECTS WHICH HAVE ONE OPTION SET TO TRUE

  let form_pipeline = vec![
    doc! {
        "$match": {
            "_id": id
        }
    },
    doc! {
        "$lookup": {
            "from": "textareas",
            "let": { "textareas": "$textareas" },
            "pipeline": [
                { "$match": {
                    "$expr": { "$and": [
                        { "$in": [ "$_id", "$$textareas" ] },
                        { "$ne": [ "$archive", true ] }
                    ]}
                }}
            ],
            "as": "textareas"
        }
    },
    doc! {
        "$lookup": {
            "from": "inputs",
            "let": { "inputs": "$inputs" },
            "pipeline": [
                { "$match": {
                    "$expr": { "$and": [
                        { "$in": [ "$_id", "$$inputs" ] },
                        { "$ne": [ "$archive", true ] }
                    ]}
                }}
            ],
            "as": "inputs"
        }
    },
    doc! {
        "$lookup": {
            "from": "selects",
            "let": { "selects": "$selects" },
            "pipeline": [
                { "$match": {
                    "$expr": { "$and": [
                        { "$in": [ "$_id", "$$selects" ] },
                        { "$ne": [ "$archive", true ] }
                    ]}
                }},
                { "$lookup": {
                    "from": "options",
                    "localField": "options",
                    "foreignField": "_id",
                    "as": "options"
                }},
                { "$match": {
                    "options.archive": { "$ne": true }
                }}
            ],
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
            "textareas": {
                "$first": "$textareas"
            },
            "selects": {
                "$push": "$selects"
            }
        }
    },
    doc! {
        "$addFields": {
            "checkboxes": {
                "$filter": {
                    "input": "$inputs",
                    "cond": {
                        "$eq": ["$$this.type_identifier", "checkbox"]
                    }
                }
            }
        }
    },
    doc! {
        "$addFields": {
            "radios": {
                "$filter": {
                    "input": "$inputs",
                    "cond": {
                        "$eq": ["$$this.type_identifier", "radio"]
                    }
                }
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
            "let": { "options": "$options" },
            "pipeline": [
                { "$match": {
                    "$expr": { "$and": [
                        { "$in": [ "$_id", "$$options" ] },
                        { "$ne": [ "$archive", true ] }
                    ]}
                }}
            ],
            "as": "options"
        }
    }
];


  let mut pipeline_value = HashMap::new();
  let _ = &mut pipeline_value.insert("select", select_pipeline);
  let _ = &mut pipeline_value.insert("form", form_pipeline);
  pipeline_value
}
