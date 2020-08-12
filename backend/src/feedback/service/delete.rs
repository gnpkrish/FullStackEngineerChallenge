use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::feedback::model::{Feedback, FeedbackResponse};
use crate::graphql::model::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub fn delete_feedback_by_id(
    context: &Context,
    feedback_id: Uuid,
) -> ServiceResult<FeedbackResponse> {
    use crate::schema::feedbacks::dsl::*;

    let conn: &PooledConnection = &context.db;
    let _feedback: Feedback = feedbacks.find(feedback_id).get_result::<Feedback>(conn)?;
    diesel::delete(feedbacks.filter(id.eq(feedback_id))).execute(conn)?;
    Ok(_feedback.into())
}
