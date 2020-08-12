use crate::comment::model::{Comment, CommentData, InsertableComment, SlimComment};
use crate::errors::ServiceResult;
use diesel::prelude::*;

pub fn add_comment(comment_data: CommentData, conn: &PgConnection) -> ServiceResult<SlimComment> {
    use crate::schema::comments::dsl::comments;

    let comment: InsertableComment = comment_data.into();
    let inserted_comment: Comment = diesel::insert_into(comments)
        .values(&comment)
        .get_result(conn)?;
    Ok(inserted_comment.into())
}
