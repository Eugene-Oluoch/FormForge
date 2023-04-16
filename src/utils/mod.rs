use serde::{Serialize, Deserialize};
use mongodb::{Client};


pub fn trim_quotes(string:&String) -> String{
  string.trim_matches('"').to_string()
}

#[derive(Clone)]
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

impl  ReturnId  {
  pub fn new(id:&str) -> Self{
    Self {
      id:id.to_string()
    }
  }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ReturnError<'a>{
  error:&'a str
}

impl <'a> ReturnError <'a> {
  pub fn new(error:&'a str) -> Self{
    Self {
      error
    }
  }
}
#[derive(Serialize,Deserialize)]
pub struct ReturnMessage<'a>{
  message:&'a str
}

impl <'a> ReturnMessage <'a>{
  pub fn new(message:&'a str) -> Self{
    Self {
      message
    }
  }
}
 