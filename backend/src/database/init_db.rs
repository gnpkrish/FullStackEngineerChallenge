use crate::database::sql_types::UserRole;
use crate::database::{db_connection, Pool};
use crate::errors::ServiceResult;
use crate::user::model::{UserData, SlimUser};
use crate::user::service::register;
use actix_web::web;

pub(crate) fn init_data(pool: web::Data<Pool>) -> ServiceResult<SlimUser> {
    let conn = &db_connection(&pool)?;
    let user1 = UserData {
        email: "admin@ers.net".to_string(),
        password: "q1w2e3r$".to_string(),
        first_name: "Administrator".to_string(),
        last_name: Some("".to_string()),
        role: UserRole::Employee,
    };
    register::create_user(user1, conn)
}
