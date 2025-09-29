use bloom_web::prelude::*;
use bloom_web::sqlx::MySqlPool;
use crate::models::{UserRepository, BloomUser};

#[get_mapping("/users/{id}")]
pub async fn get_bloom_user_by_id(path: web::Path<i64>, pool: web::Data<MySqlPool>) -> impl bloom_web::actix_web::Responder {
    match UserRepository::find_by_id::<BloomUser>(pool.get_ref(), path.into_inner()).await {
        Ok(Some(user)) => HttpResponse::Ok().json(user),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}