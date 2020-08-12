use crate::comment::model::Comment;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub fn filter_comments(
    context: &Context,
    _feedback_id: Uuid,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Comment>> {
    use crate::schema::comments::dsl::*;
    let conn: &PooledConnection = &context.db;

    Ok(comments
        .filter(feedback_id.eq(_feedback_id))
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Comment>(conn)?)
}
