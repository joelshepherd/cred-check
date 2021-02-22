create table "source" (
  id serial primary key,
  title text not null,
  url text not null unique,
  created_at timestamp with time zone not null default now()
);

create table "opinion" (
  id serial primary key,
  user_id int not null,
  source_id int not null,
  position boolean not null,
  body text not null,
  created_at timestamp with time zone not null default now()
);

create index opinion_source_id_idx on "opinion" (source_id);

create table "user" (
  id serial primary key,
  username text not null unique,
  name text not null,
  created_at timestamp with time zone not null default now()
);

create table "vote" (
  id serial primary key,
  user_id int not null,
  source_id int not null,
  opinion_id int not null,
  position boolean not null,
  created_at timestamp with time zone not null default now(),
  unique (source_id, user_id)
);

create index vote_source_id_idx on "vote" (source_id);