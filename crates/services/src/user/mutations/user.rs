use crate::auth::jwt::JWT;
use bcrypt::DEFAULT_COST;
use database::entities::user::{ActiveModel, Column, Entity, Model};
use sea_orm::ActiveValue::Set;
use sea_orm::*;
use serde::{Deserialize, Serialize};

pub struct UserMutations;

#[derive(Serialize, Deserialize)]
pub struct SignIn {
    token: String,
    token_type: String,
    user_id: i32,
    username: String,
}

impl UserMutations {
    pub async fn create(username: String, password: String, db: &DbConn) -> Result<Model, DbErr> {
        let user_exist = Entity::find()
            .filter(Column::Username.contains(&username))
            .one(db)
            .await?;

        if user_exist.is_some() {
            return Err(DbErr::Custom(format!(
                "Already exist an user with the username {}.",
                username
            )));
        }

        let hashed_password = bcrypt::hash(&password, DEFAULT_COST).unwrap();
        let user = ActiveModel {
            username: Set(username.to_owned()),
            password: Set(hashed_password),
            ..Default::default()
        };

        let user_created = Entity::insert(user).exec(db).await?;
        Ok(Model {
            id: user_created.last_insert_id,
            username: username.to_owned(),
            password: password.to_owned(),
        })
    }

    pub async fn sign_in(username: String, password: String, db: &DbConn) -> Result<SignIn, DbErr> {
        let user = Entity::find()
            .filter(Column::Username.contains(username))
            .one(db)
            .await?;

        if user.is_none() {
            return Err(DbErr::RecordNotFound(
                "Cannot find an user with these credentials.".to_string(),
            ));
        };

        let user = user.unwrap();

        let password_try = bcrypt::verify(&password, &user.password);
        if password_try.is_err() {
            return Err(DbErr::Custom("Invalid password.".to_string()));
        }

        let token = JWT::encode(&user.id);
        if token.is_err() {
            return Err(DbErr::Custom(
                "An error occurred when creating auth token.".to_string(),
            ));
        }

        Ok(SignIn {
            token_type: "Bearer".to_string(),
            token: token.unwrap(),
            username: user.username,
            user_id: user.id,
        })
    }
}
