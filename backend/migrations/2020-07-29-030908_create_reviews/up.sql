CREATE TABLE feedbacks(
  id UUID PRIMARY KEY NOT NULL,
  user_id UUID NOT NULL,
  feedback TEXT NOT NULL,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_feedbacks_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
CREATE TABLE participants(
  user_id UUID NOT NULL,
  feedback_id UUID NOT NULL,
  PRIMARY KEY (user_id, feedback_id),
  CONSTRAINT fk_participants_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_participants_feedback_id FOREIGN KEY (feedback_id) REFERENCES feedbacks(id) ON DELETE CASCADE
);
