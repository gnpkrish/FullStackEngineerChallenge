use crate::feedback::model::Feedback;
use crate::schema::participants;
use crate::user::model::User;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Associations, Queryable, PartialEq)]
#[belongs_to(User)]
#[belongs_to(Feedback)]
#[table_name = "participants"]
pub struct Participant {
    pub user_id: Uuid,
    pub feedback_id: Uuid,
}

#[derive(Debug, Deserialize, juniper::GraphQLInputObject)]
pub struct ParticipantData {
    pub user_id: Uuid,
    pub feedback_id: Uuid,
}

#[derive(Debug, Insertable)]
#[table_name = "participants"]
pub struct InsertableParticipant {
    pub user_id: Uuid,
    pub feedback_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize, Clone, juniper::GraphQLObject)]
pub struct SlimParticipant {
    pub user_id: Uuid,
    pub feedback_id: Uuid,
}

impl From<ParticipantData> for InsertableParticipant {
    fn from(participant_data: ParticipantData) -> Self {
        let ParticipantData {
            user_id,
            feedback_id,
            ..
        } = participant_data;
        Self {
            user_id,
            feedback_id,
        }
    }
}
impl From<Participant> for SlimParticipant {
    fn from(participant: Participant) -> Self {
        let Participant {
            user_id,
            feedback_id,
            ..
        } = participant;
        Self {
            user_id,
            feedback_id,
        }
    }
}
