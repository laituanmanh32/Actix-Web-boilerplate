-- Your SQL goes here


CREATE TABLE "todo"
(
    id    bigserial PRIMARY KEY,
    title TEXT not null,
    status varchar(10) not null ,
    created_at TIMESTAMPTZ not null Default NOW()::timestamptz,
    updated_at TIMESTAMPTZ
)