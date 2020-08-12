use crate::errors::ServiceResult;
use crate::participant::model::{
    InsertableParticipant, Participant, ParticipantData, SlimParticipant,
};
use diesel::prelude::*;

pub fn add_participant(
    participant_data: ParticipantData,
    conn: &PgConnection,
) -> ServiceResult<SlimParticipant> {
    use crate::schema::participants::dsl::participants;

    let participant: InsertableParticipant = participant_data.into();
    let inserted_participant: Participant = diesel::insert_into(participants)
        .values(&participant)
        .get_result(conn)?;
    Ok(inserted_participant.into())
}
