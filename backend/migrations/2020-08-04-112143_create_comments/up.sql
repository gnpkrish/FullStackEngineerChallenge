CREATE TABLE comments(
  id UUID PRIMARY KEY NOT NULL,
  user_id UUID NOT NULL,
  feedback_id UUID NOT NULL,
  body TEXT NOT NULL,
  submited_at TIMESTAMP NOT NULL DEFAULT NOW(),
  CONSTRAINT fk_comments_user_id FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
  CONSTRAINT fk_comments_feedback_id FOREIGN KEY (feedback_id) REFERENCES feedbacks(id) ON DELETE CASCADE
);