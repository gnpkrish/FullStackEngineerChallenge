use crate::database::sql_types::UserRole;
use crate::errors::ServiceResult;
use crate::user::model::{EditableUser, SlimUser, User};
use diesel::prelude::*;
use uuid::Uuid;

pub fn modify_user(
    conn: &PgConnection,
    user_id: Uuid,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    role: Option<UserRole>,
) -> ServiceResult<SlimUser> {
    use crate::schema::users;
    let editable_user: EditableUser = EditableUser {
        first_name,
        last_name,
        email,
        role,
    };

    let user: User = diesel::update(users::table.find(user_id))
        .set(&editable_user)
        .get_result(conn)?;
    Ok(user.into())
}
