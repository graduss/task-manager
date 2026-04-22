-- Add migration script here
CREATE TYPE event_status AS ENUM ('pending', 'processing', 'completed');
CREATE TYPE event_type AS ENUM ('createtask', 'updatetask', 'deletetask');

CREATE TABLE events (
    id          UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    task_id     UUID NOT NULL REFERENCES tasks(id) ON DELETE CASCADE,
    status      event_status NOT NULL DEFAULT 'pending',
    type        event_type NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at  TIMESTAMPTZ NOT NULL DEFAULT NOW()
);