use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::graphql::model::Context;
use crate::schema::*;
use crate::user::model::User;
use chrono::*;
use diesel::prelude::*;
use diesel::sql_query;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Identifiable, Queryable, PartialEq, Associations)]
#[belongs_to(User)]
#[table_name = "feedbacks"]
pub struct Feedback {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feedback: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct FeedbackData {
    pub user_id: Uuid,
    pub feedback: String,
}

#[derive(Debug, Insertable)]
#[table_name = "feedbacks"]
pub struct InsertableFeedback {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feedback: String,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct FeedbackResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub feedback: String,
    pub created_at: NaiveDateTime,
    // pub participants: Vec<ParticipantData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, QueryableByName, juniper::GraphQLObject)]
#[table_name = "users"]
pub struct FeedbackParticipants {
    pub id: Uuid,
    pub email: String,
    pub first_name: String,
    pub last_name: Option<String>,
}

#[juniper::object(Context = Context)]
impl Feedback {
    pub fn id(&self) -> Uuid {
        self.id
    }
    pub fn user_id(&self) -> Uuid {
        self.user_id
    }
    pub fn feedback(&self) -> &str {
        self.feedback.as_str()
    }
    pub fn created_at(&self) -> NaiveDateTime {
        self.created_at
    }
    pub fn participants(&self, context: &Context) -> ServiceResult<Vec<FeedbackParticipants>> {
        use crate::schema::feedbacks::dsl::*;
        let conn: &PooledConnection = &context.db;
        let _feedback = feedbacks.find(self.id).get_result::<Feedback>(conn)?;
        let users = sql_query(format!(
            "
            SELECT u.id, u.email,
            u.first_name, u.last_name
            from users u
            JOIN participants p ON u.id = p.user_id
            WHERE p.feedback_id = '{}'
        ",
            self.id
        ))
        .load::<FeedbackParticipants>(conn)?;
        Ok(users)
    }
}

impl From<FeedbackData> for InsertableFeedback {
    fn from(feedback_data: FeedbackData) -> Self {
        let FeedbackData {
            user_id, feedback, ..
        } = feedback_data;

        Self {
            id: Uuid::new_v4(),
            user_id,
            feedback,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}

impl From<Feedback> for FeedbackResponse {
    fn from(feedback: Feedback) -> Self {
        let Feedback {
            id,
            user_id,
            feedback,
            created_at,
            ..
        } = feedback;

        Self {
            id,
            user_id,
            feedback,
            created_at,
        }
    }
}
