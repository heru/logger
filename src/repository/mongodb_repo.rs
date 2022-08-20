use std::env;
extern crate dotenv;
use dotenv::dotenv;

use mongodb::{
  bson::{
    doc,
    extjson::de::Error,
    
  },
  results::{ InsertOneResult, UpdateResult },
  Client, 
  Collection,
};
use mongodb::bson::oid::ObjectId;
use crate::models::user_model::User;

pub struct MongoRepo {
  col: Collection<User>,
}

impl MongoRepo {
  pub async fn init() -> Self {
    dotenv().ok();
    let uri = match env::var("MONGOURI") {
      Ok(v) => v.to_string(),
      Err(_) => format!("Error loading env variable")
    };

    let client = Client::with_uri_str(uri).await.unwrap();
    let db = client.database("logsDB");
    let col: Collection<User> = db.collection("user");
    MongoRepo { col }
  }

  pub async fn create_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
    let new_doc = User {
      name: new_user.name,
      location: new_user.location,
      title: new_user.title
    };

    let user = self
      .col
      .insert_one(new_doc, None)
      .await
      .ok()
      .expect("error create user brooo.");

    Ok(user)
  }

  
  pub async fn get_user(&self, name: &String) -> Result<User, Error> {
    // let obj_id = ObjectId::parse_str(name);
    // let filter = doc! { "_id": &obj_id };
    let user_detail = self
      .col
      .find_one(doc! { "name": &name }, None)
      .await
      .ok()
      .expect("error getting user's detailt");

    Ok(user_detail.unwrap())
  }

  pub async fn update_user(&self, id: &String, new_user: User) -> Result<UpdateResult, Error> {
    let obj_id = ObjectId::parse_str(id).unwrap();
    let filter = doc! { "_id": obj_id };
    let new_doc = doc! {
      "$set": 
        {
          "id": new_user.id,
          "name": new_user.name,
          "location": new_user.location,
          "title": new_user.title
        }
    };
    let updated_doc = self
      .col
      .update_one(filter, new_doc, None)
      .await
      .ok()
      .expect("Error updating user");
    Ok(updated_doc)
  }
}