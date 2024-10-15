use crate::domain::{models::{Country, User}, usecase::UseCase};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct UserData {
    pub user: User,
    pub country: Country,
}

#[derive(Serialize)]
pub struct GetAllUsersResponse {
    pub user_data_list: Vec<UserData>,
}

pub struct GetAllUsersUseCase;

impl UseCase<GetAllUsersResponse> for GetAllUsersUseCase {
    fn execute(&self) -> GetAllUsersResponse {
        let country_id = Uuid::new_v4();
        let user_data = UserData {
            user: User {
                country_id: country_id,
                email: "ole@helvete.no".to_string(),
                first_name: "ole".to_string(),
                last_name: "helvete".to_string(),
                phone: Some("93021759".to_string()),
                id: Uuid::new_v4(),
            },
            country: Country {
                id: country_id,
                iso_alpha_2: "NO".to_string(),
                iso_alpha_3: "NOR".to_string(),
                iso_name: "Norway".to_string(),
            }
        };
        GetAllUsersResponse {
            user_data_list: vec![user_data],
        }
    }
}


