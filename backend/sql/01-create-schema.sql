-- Item status enum
CREATE TYPE grocery_status_enum AS ENUM (
    'shelf',
    'basket'
);

-- Groceries
CREATE TABLE groceries (
    id bigserial,
    cid bigint NOT NULL, -- creator user id
    cost bigint NOT NULL DEFAULT 0,
    name text NOT NULL,
    status grocery_status_enum NOT NULL DEFAULT 'shelf'
);
ALTER SEQUENCE groceries_id_seq RESTART WITH 1000;