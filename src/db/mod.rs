use std::fmt::format;

use mongodb::{Client, Collection, results::{InsertOneResult}, bson::{doc, Bson, Document}, error::Error};
use rocket::serde::DeserializeOwned;
use serde::Serialize;
use crate::OptionSelect;
use crate::utils::string_to_object_id;
use dotenv::dotenv;
use std::env::var;




async fn create_collection<T>(client:&Client,db_name:&str, collection:&str) -> Collection<T>{
  client.database("crabs_test").collection(collection)
}

pub async fn create_connection() -> Client {
  // LOADS ENVS
  dotenv().ok();
  let (mongo_username,mongo_password,mongo_cluster) =(
    var("MONGO_USERNAME").expect("MONGO_USERNAME must be set."),
    var("MONGO_PASSWORD").expect("MONGO_PASSWORD must be set."),
    var("MONGO_CLUSTER").expect("MONGO_CLUSTER must be set.")
  );

  // MONGO URI
  let uri = format!("mongodb+srv://{}:{}@{}.bh0z6ws.mongodb.net/?retryWrites=true&w=majority",mongo_username,mongo_password,mongo_cluster);
  
  Client::with_uri_str(uri).await.expect("Failed to initialize client.")
}


pub async fn insert_doc<T>(client:&Client,db_name:&str, collection:&str,document:&T) -> Result<InsertOneResult, String>
where T: Serialize
{
  let col:Collection<T> = create_collection(client, db_name, collection).await;
  match col.insert_one(document,None).await{
    Ok(column) => Ok(column),
    Err(_error) => Err(String::from("Failed"))
  }

}

pub async fn get_by_id<T>(client:&Client,db_name:&str,collection:&str,id:&str) -> Result<Option<T>,Error>
where T:Serialize 
+ std::fmt::Debug 
+ DeserializeOwned 
+ for<'de> serde::Deserialize<'de> 
+ Unpin + Send + Sync
{
  let col:Collection<T> = create_collection(client, db_name, collection).await;
  col.find_one(doc! {"_id": string_to_object_id(id)},None).await
}


// GET DOCUMENT AND ALL RELATIONSHIP REFERENCE
pub async fn get_all<T>(client:&Client,db_name:&str,collection:&str,pipeline:Vec<Document>) -> Document{
  let col:Collection<T> = create_collection(client, db_name, collection).await;
  col.aggregate(pipeline, None).await.unwrap().deserialize_current().unwrap()
}

// UPDATES WILL BE MERGED TO ONE -> NOTE
pub async fn update_doc<T>(client:&Client,db_name:&str, collection:&str,option:T, id:&str)
where Bson: From<T>
{
  let col:Collection<T> = create_collection(client, db_name, collection).await;
  let update = doc! {"$set": option};
  update_query(&col, update, id).await;
}

// METHOD TO PUSH NEW ITEM TO EXISTING ARRAY
pub async fn update_push<T>(client:&Client,db_name:&str,collection:&str,document:Document,id:&str)
where Bson: From<T>
{
  let col:Collection<T> = create_collection(client, db_name, collection).await;
  update_query(&col, document, id).await;
}

// METHOD TO REMOVE ITEM FROM EXISTING


pub async fn update_query<T>(col:&Collection<T>,update:Document,id:&str){
  println!("{:?}",col
  .update_one(doc! {"_id": string_to_object_id(id)}, update, None).await.unwrap());
}




pub async fn delete_by_id(client:&Client,db_name:&str, collection:&str, id:&str) {
  let col:Collection<OptionSelect> = create_collection(client, db_name, collection).await;
  println!("{:?}",&col.delete_one(doc! {"_id":string_to_object_id(id)}, None).await);
}
