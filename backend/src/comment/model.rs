use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::feedback::model::Feedback;
use crate::graphql::model::Context;
use crate::schema::*;
use crate::user::model::User;
use chrono::*;
use uuid::Uuid;

use diesel::prelude::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[belongs_to(Feedback)]
pub struct Comment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feedback_id: Uuid,
    pub body: String,
    pub submited_at: NaiveDateTime,
}

#[juniper::object(Context = Context)]
impl Comment {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    pub fn feedback_id(&self) -> Uuid {
        self.feedback_id
    }
    pub fn body(&self) -> &str {
        self.body.as_str()
    }
    pub fn submited_at(&self) -> NaiveDateTime {
        self.submited_at
    }
    pub fn user(&self, context: &Context) -> ServiceResult<User> {
        use crate::schema::users::dsl::*;
        let conn: &PooledConnection = &context.db;
        Ok(users.find(self.user_id).get_result::<User>(conn)?)
    }

    pub fn feedback(&self, context: &Context) -> ServiceResult<Feedback> {
        use crate::schema::feedbacks::dsl::*;
        let conn: &PooledConnection = &context.db;
        Ok(feedbacks
            .find(self.feedback_id)
            .get_result::<Feedback>(conn)?)
    }
}
#[derive(Debug, Insertable)]
#[table_name = "comments"]
pub struct InsertableComment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feedback_id: Uuid,
    pub body: String,
    pub submited_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct CommentData {
    pub user_id: Uuid,
    pub feedback_id: Uuid,
    pub body: String,
}

impl From<CommentData> for InsertableComment {
    fn from(comment_data: CommentData) -> Self {
        let CommentData {
            user_id,
            feedback_id,
            body,
            ..
        } = comment_data;
        Self {
            id: Uuid::new_v4(),
            user_id,
            feedback_id,
            body,
            submited_at: chrono::Local::now().naive_local(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimComment {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feedback_id: Uuid,
    pub body: String,
    pub submited_at: NaiveDateTime,
}

impl From<Comment> for SlimComment {
    fn from(comment_data: Comment) -> Self {
        let Comment {
            id,
            user_id,
            feedback_id,
            body,
            submited_at,
            ..
        } = comment_data;
        Self {
            id,
            user_id,
            feedback_id,
            body,
            submited_at,
        }
    }
}
