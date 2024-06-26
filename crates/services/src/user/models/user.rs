use database::entities::user::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserModel {
    pub id: i32,
    pub username: String,
    pub tasks: Vec<database::entities::task::Model>,
}

impl From<Option<&(Model, Vec<database::entities::task::Model>)>> for UserModel {
    fn from(v: Option<&(Model, Vec<database::entities::task::Model>)>) -> Self {
        let (user, tasks) = v.unwrap().clone();

        Self {
            id: user.id,
            username: user.username,
            tasks,
        }
    }
}
