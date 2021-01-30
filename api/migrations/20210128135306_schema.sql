create table source (
  id serial primary key,
  url varchar(255) not null unique
);

create table opinion (
  id serial primary key ,
  source_id int not null,
  position boolean not null,
  body text not null
);