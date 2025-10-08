CREATE TABLE boat_owners (
    boat_id UUID NOT NULL REFERENCES boats(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (boat_id, user_id)
);
