use mongodb::bson;
use rocket::State;
use crate::utils::StateCustom;
use crate::db::get_all;
use crate::{Select};
use crate::repository::map;



#[get("/<id>")]
pub async fn get_select_by_id(id:&str,client:&State<StateCustom>) -> String{
  let c = get_all::<Select>(&client.client, "crabs_test", "selects", map("select",id)).await; 
  serde_json::to_string(&c).expect("Failed to serialise")
}