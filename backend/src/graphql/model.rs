use crate::cli_args::Opt;
use crate::comment::model::{Comment, CommentData, SlimComment};
use crate::comment::service as comment;
use crate::database::sql_types::UserRole;
use crate::database::PooledConnection;
use crate::errors::ServiceResult;
use crate::feedback::model::{Feedback, FeedbackData, FeedbackResponse};
use crate::feedback::service as feedback;
use crate::jwt::model::{DecodedToken, Token};
use crate::participant::model::{ParticipantData, SlimParticipant};
use crate::user::model::{LoggedUser, SlimUser, User, UserData};
use crate::user::service as user;
use crate::user::service::token::ClaimsResponse;

use diesel::PgConnection;
use juniper::Context as JuniperContext;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Clone)]
pub struct Context {
    pub opt: Opt,
    pub db: Arc<PooledConnection>,
    pub user: LoggedUser,
    pub token: DecodedToken,
}

impl JuniperContext for Context {}

impl Context {
    pub fn new(token: DecodedToken, user: LoggedUser, pool: PooledConnection, opt: Opt) -> Self {
        Self {
            opt,
            token,
            user,
            db: Arc::new(pool),
        }
    }
}

pub(crate) struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    pub fn users(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<User>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        crate::user::has_role(&context.user, &UserRole::Employee)?;

        user::list::find_all_users(&context, limit, offset)
    }

    pub fn generate_token(context: &Context) -> ServiceResult<Token> {
        user::token::generate(&context)
    }

    pub fn decode_token(context: &Context) -> ServiceResult<&ClaimsResponse> {
        user::token::decode(&context)
    }

    pub fn feedbacks(
        context: &Context,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Feedback>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);

        // crate::user::has_role(&context.user, &UserRole::Employee)?;

        feedback::list::find_all_feedbacks(&context, limit, offset)
    }

    pub fn get_feedback_by_id(context: &Context, feedback_id: Uuid) -> ServiceResult<Feedback> {
        feedback::find::find_by_id(context, feedback_id)
    }
    pub fn comments_for_feedback(
        context: &Context,
        feedback_id: Uuid,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> ServiceResult<Vec<Comment>> {
        let limit: i32 = limit.unwrap_or(100);
        let offset: i32 = offset.unwrap_or(0);
        comment::filter::filter_comments(context, feedback_id, limit, offset)
    }
}

pub(crate) struct Mutation;

#[juniper::object(Context = Context)]
impl Mutation {
    pub fn register(context: &Context, data: UserData) -> ServiceResult<SlimUser> {
        use crate::user::service::register::create_user;
        let conn: &PgConnection = &context.db;

        Ok(create_user(data, conn)?)
    }

    pub fn update_user(
        context: &Context,
        user_id: Uuid,
        first_name: Option<String>,
        last_name: Option<String>,
        email: Option<String>,
        role: Option<UserRole>,
    ) -> ServiceResult<SlimUser> {
        let conn: &PgConnection = &context.db;
        Ok(user::update::modify_user(
            conn, user_id, first_name, last_name, email, role,
        )?)
    }

    pub fn delete_user(context: &Context, user_id: Uuid) -> ServiceResult<SlimUser> {
        Ok(user::delete::delete_user_by_id(context, user_id)?)
    }

    pub fn create_feedback(
        context: &Context,
        data: FeedbackData,
    ) -> ServiceResult<FeedbackResponse> {
        use crate::feedback::service::create::add_feedback;
        let conn: &PgConnection = &context.db;

        Ok(add_feedback(data, conn)?)
    }

    pub fn delete_feedback(
        context: &Context,
        feedback_id: Uuid,
    ) -> ServiceResult<FeedbackResponse> {
        Ok(feedback::delete::delete_feedback_by_id(
            context,
            feedback_id,
        )?)
    }

    pub fn assign_participant(
        context: &Context,
        data: ParticipantData,
    ) -> ServiceResult<SlimParticipant> {
        use crate::participant::service::create::add_participant;
        let conn: &PgConnection = &context.db;

        Ok(add_participant(data, conn)?)
    }

    pub fn create_comment(context: &Context, data: CommentData) -> ServiceResult<SlimComment> {
        let conn: &PgConnection = &context.db;

        Ok(comment::create::add_comment(data, conn)?)
    }
}

pub(crate) type Schema = juniper::RootNode<'static, QueryRoot, Mutation>;

pub(crate) fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, Mutation {})
}
