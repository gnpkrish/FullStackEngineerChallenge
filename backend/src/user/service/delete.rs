use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::user::model::{SlimUser, User};
use diesel::prelude::*;
use uuid::Uuid;

pub fn delete_user_by_id(context: &Context, user_id: Uuid) -> ServiceResult<SlimUser> {
    use crate::schema::users::dsl::*;

    let conn: &PooledConnection = &context.db;
    let user: User = users.find(user_id).get_result::<User>(conn)?;
    diesel::delete(users.filter(id.eq(user_id))).execute(conn)?;
    Ok(user.into())
}
