use crate::auth::jwt::JWT;
use crate::user::models::user::UserModel;
use database::entities::user::{Column, Entity};
use sea_orm::*;

pub struct UserQueries;

impl UserQueries {
    pub async fn get_current_user(user: JWT, db: &DbConn) -> Result<UserModel, DbErr> {
        let user: UserModel = Entity::find()
            .filter(Column::Id.eq(user.claims.sub))
            .find_with_related(database::entities::task::Entity)
            .all(db)
            .await?
            .first()
            .into();

        Ok(user)
    }
}
