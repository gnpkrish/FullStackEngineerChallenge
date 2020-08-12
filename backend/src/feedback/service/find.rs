use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::feedback::model::Feedback;
use crate::graphql::model::Context;
use diesel::prelude::*;
use uuid::Uuid;

pub(crate) fn find_by_id(context: &Context, feedback_id: Uuid) -> ServiceResult<Feedback> {
    use crate::schema::feedbacks::dsl::*;
    let conn: &PooledConnection = &context.db;
    // let feedback: Feedback = feedbacks
    //     .find(feedback_id)
    //     .get_result::<Feedback>(conn)?;
    // print!("{:#?}", feedback);
    Ok(feedbacks.find(feedback_id).get_result::<Feedback>(conn)?)
}
