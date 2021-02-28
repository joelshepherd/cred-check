create table "source" (
  id bigserial primary key,
  title text not null,
  canonical_url text not null unique,
  created_at timestamp with time zone not null default now()
);

create table "alternative" (
  id bigserial primary key,
  source_id bigint not null,
  url text not null unique
);

create table "opinion" (
  id bigserial primary key,
  user_id bigint not null,
  source_id bigint not null,
  position boolean not null,
  body text not null,
  created_at timestamp with time zone not null default now()
);

create index opinion_source_id_idx on "opinion" (source_id);

create table "user" (
  id bigserial primary key,
  username text not null unique,
  name text not null,
  created_at timestamp with time zone not null default now()
);

create table "vote" (
  id bigserial primary key,
  user_id bigint not null,
  source_id bigint not null,
  opinion_id bigint not null,
  position boolean not null,
  created_at timestamp with time zone not null default now(),
  unique (source_id, user_id)
);

create index vote_opinion_id_idx on "vote" (opinion_id);

create index vote_source_id_idx on "vote" (source_id);