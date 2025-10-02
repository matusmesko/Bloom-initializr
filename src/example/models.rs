use bloom_web::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Entity, Debug, Clone, Serialize, Deserialize, FromRow)]
#[table("bloom_users")]
#[allow(dead_code)]
pub struct BloomUser {
    #[id]
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[repository(BloomUser)]
pub struct UserRepository;

impl UserRepository {
    #[query("SELECT * FROM users WHERE name = ?")]
    pub async fn find_by_name(pool: &MySqlPool, name: String) -> anyhow::Result<Vec<User>> {}
}