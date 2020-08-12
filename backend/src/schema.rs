table! {
    comments (id) {
        id -> Uuid,
        user_id -> Uuid,
        feedback_id -> Uuid,
        body -> Text,
        submited_at -> Timestamp,
    }
}

table! {
    feedbacks (id) {
        id -> Uuid,
        user_id -> Uuid,
        feedback -> Text,
        created_at -> Timestamp,
    }
}

table! {
    participants (user_id, feedback_id) {
        user_id -> Uuid,
        feedback_id -> Uuid,
    }
}

table! {
    users (id) {
        id -> Uuid,
        hash -> Bytea,
        salt -> Varchar,
        email -> Varchar,
        first_name -> Varchar,
        last_name -> Nullable<Varchar>,
        role -> Varchar,
        created_at -> Timestamp,
    }
}

joinable!(comments -> feedbacks (feedback_id));
joinable!(comments -> users (user_id));
joinable!(feedbacks -> users (user_id));
joinable!(participants -> feedbacks (feedback_id));
joinable!(participants -> users (user_id));

allow_tables_to_appear_in_same_query!(comments, feedbacks, participants, users,);
