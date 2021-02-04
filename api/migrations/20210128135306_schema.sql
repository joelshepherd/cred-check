create table "source" (
  id serial primary key,
  title text not null,
  url text not null unique
);

create table "opinion" (
  id serial primary key,
  user_id int not null,
  source_id int not null,
  position boolean not null,
  body text not null
);

create table "user" (
  id serial primary key,
  username text not null unique,
  name text not null
);

create table "supporter" (
  id serial primary key,
  user_id int not null,
  opinion_id int not null
);