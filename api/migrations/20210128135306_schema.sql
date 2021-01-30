create table "source" (
  id serial primary key,
  title varchar not null,
  url varchar not null unique
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
  name varchar not null
);

create table "supporter" (
  id serial primary key,
  user_id int not null,
  opinion_id int not null
);