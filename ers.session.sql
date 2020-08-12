SELECT * FROM users;
SELECT * FROM feedbacks;
SELECT * FROM participants;
SELECT uuid_generate_v4();
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

INSERT INTO users (id, email, first_name, role) VALUES (uuid_generate_v4(), "admin@ers.net", "Administrator", "Admin");