use rocket::{serde::json::Json, State};

use crate::{
  models::{
    validate::{
      Validate
    }
  }, 
  utils::{
    StateCustom
  },
  views::{
    validate::{validate_data_view}
  }
};

#[post("/post",data="<data>")]
pub async fn validate_data(data:Json<Validate>,client:&State<StateCustom>){
  validate_data_view(data.0, &client.client).await;
}