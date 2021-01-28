create table source (
  id int primary key not null,
  url varchar(255) not null
);

create table opinion (
  id int primary key not null,
  source_id int not null,
  position boolean not null,
  body text not null
);