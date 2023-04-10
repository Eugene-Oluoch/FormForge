use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use mongodb::{Client};


pub fn string_to_object_id(id:&str) -> ObjectId{
  ObjectId::parse_str(&id.to_string()).unwrap()
}

pub struct StateCustom{
  pub client:Client
}

impl StateCustom {
  pub fn new(client:Client) -> Self{
    Self {
      client:client
    }
  }
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnId {
  id:String
}

impl ReturnId {
  pub fn new(id:String) -> Self{
    Self {
      id
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnError{
  error:String
}

impl  ReturnError {
  pub fn new(error:String) -> Self{
    Self {
      error
    }
  }
}