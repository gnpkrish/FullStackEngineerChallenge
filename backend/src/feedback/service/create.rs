use crate::errors::ServiceResult;
use crate::feedback::model::{Feedback, FeedbackData, FeedbackResponse, InsertableFeedback};
use diesel::prelude::*;

pub fn add_feedback(
    feedback_data: FeedbackData,
    conn: &PgConnection,
) -> ServiceResult<FeedbackResponse> {
    use crate::schema::feedbacks::dsl::feedbacks;

    let feedback: InsertableFeedback = feedback_data.into();
    let inserted_feedback: Feedback = diesel::insert_into(feedbacks)
        .values(&feedback)
        .get_result(conn)?;
    Ok(inserted_feedback.into())
}
