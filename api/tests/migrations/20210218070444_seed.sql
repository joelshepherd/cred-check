insert into
  source (title, url)
values
  ('Test title', 'test.com');

insert into
  "user" (name, username)
values
  ('Test', 'test');

insert into
  opinion (position, source_id, user_id, body)
values
  (true, 1, 1, 'Test opinion body');