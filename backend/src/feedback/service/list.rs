use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::feedback::model::Feedback;
use crate::graphql::model::Context;
use diesel::prelude::*;

pub(crate) fn find_all_feedbacks(
    context: &Context,
    limit: i32,
    offset: i32,
) -> ServiceResult<Vec<Feedback>> {
    use crate::schema::feedbacks::dsl::*;
    let conn: &PooledConnection = &context.db;
    let feedbacks_vec: Vec<Feedback> = feedbacks
        .limit(limit as i64)
        .offset(offset as i64)
        .load::<Feedback>(conn)?;
    Ok(feedbacks_vec)
}
