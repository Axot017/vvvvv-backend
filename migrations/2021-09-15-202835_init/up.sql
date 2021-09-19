-- Your SQL goes here

CREATE TABLE "profile" (
  "id" BIGSERIAL PRIMARY KEY NOT NULL,
  "name" varchar(48) NOT NULL UNIQUE,
  "email" varchar(128) NOT NULL UNIQUE,
  "password" varchar(128) NOT NULL,
  "role" varchar(48) NOT NULL DEFAULT 'USER',
  "avatar_id" varchar(48),
  "birthday" timestamp with TIME ZONE,
  "email_confirmed_at" timestamp with TIME ZONE,
  "updated_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW(),
  "created_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE "category" (
  "id" BIGSERIAL PRIMARY KEY NOT NULL,
  "image_id" varchar(48),
  "name" varchar(128) NOT NULL,
  "updated_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW(),
  "created_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE TABLE "post" (
  "id" BIGSERIAL PRIMARY KEY NOT NULL,
  "profile_id" bigint NOT NULL REFERENCES "profile",
  "category_id" bigint NOT NULL REFERENCES "category",
  "title" varchar(255) NOT NULL,
  "resource_id" varchar(48) NOT NULL,
  "description" text,
  "approved_at" timestamp with TIME ZONE,
  "updated_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW(),
  "created_at" timestamp with TIME ZONE NOT NULL DEFAULT NOW()
);

SELECT diesel_manage_updated_at('post');

SELECT diesel_manage_updated_at('category');

SELECT diesel_manage_updated_at('profile');

CREATE UNIQUE INDEX idx_email on "profile"("email");

CREATE UNIQUE INDEX idx_username on "profile"("name");
