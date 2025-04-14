use crate::{
    domain::{models::User, usecase::UseCase},
    get_users,
    infrastructure::user_repository,
};
use serde::Serialize;
use sqlx::PgPool;

#[derive(Serialize)]
pub struct UserData {
    pub user: User,
    //pub country: Country,
}

#[derive(Serialize)]
pub struct GetAllUsersResponse {
    pub user_data_list: Vec<User>,
}

pub struct GetAllUsersUseCase;

impl UseCase<GetAllUsersResponse> for GetAllUsersUseCase {
    async fn execute(&self, pool: PgPool) -> GetAllUsersResponse {
        let users = user_repository::get_users(&pool).await.unwrap();
        GetAllUsersResponse {
            user_data_list: users,
        }
    }
}
