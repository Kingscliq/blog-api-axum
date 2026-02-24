-- Add migration script here

CREATE TABLE IF NOT EXISTS posts (
   id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
   title TEXT NOT NULL,
   image_url TEXT NOT NULL,
   content TEXT NOT NULL,
   description TEXT NOT NULL,
   created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
   updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
