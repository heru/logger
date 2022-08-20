use crate::{
  models::user_model::User,
  repository::mongodb_repo::MongoRepo,
};

use actix_web::{
  post,
  get,
  put,
  web::{Data, Json, Path },
  HttpResponse
};

#[post("/user")]
pub async fn create_user(db: Data<MongoRepo>, new_user: Json<User>) -> HttpResponse {
  let data = User{
    name: new_user.name.to_owned(),
    location: new_user.location.to_owned(),
    title: new_user.title.to_owned()
  };

  let user_detail = db.create_user(data).await;
  match user_detail {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}

#[get("/user/{name}")]
pub async fn get_user(db: Data<MongoRepo>, name: Path<String>) -> HttpResponse {
  let user_name = name.into_inner();
  if user_name.is_empty() {
    return HttpResponse::BadRequest().body("Invalid id");
  }

  let user_detail = db.get_user(&user_name).await;
  match user_detail {
    Ok(user) => HttpResponse::Ok().json(user),
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}

#[put("/user/{id}")]
pub async fn update_user(
  db: Data<MongoRepo>,
  path: Path<String>,
  new_user: Json<User>,
) -> HttpResponse {
  let id = path.into_inner();
  if id.is_empty() {
    return HttpResponse::BadRequest().body("Invalid Request");
  };

  let data = User {
    id: Some(ObjectId::parse_str(&id).unwrap()),
    name: new_user.name.to_owned(),
    location: new_user.location.to_owned(),
    title: new_user.title.to_owned()
  };
  let update_result = db.update_user(&id, data).await;
  match update_result {
    Ok(update) => {
      if update.match_count == 1 {
        let updated_user_info = db.get_user(&id).await;
        return match updated_user_info {
            Ok(user) => HttpResponse::Ok().json(user),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
        };
      } else {
        return HttpResponse::NotFound().body("No user found with specified ID");
      }
    }
    Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
  }
}