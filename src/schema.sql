create table Event (
  id                  serial    primary key,
  tournament          boolean   not null,
  title               text      not null,
  date_time           timestamp not null,
  unconfirmed_players integer[] not null,
  confirmed_players   integer[] not null,
  active              boolean   not null
);

create table "User" (
  id            serial primary key,
  steam_id      text   not null,
  name          text   not null,
  email         text   not null,
  player_tag    text,
  rank          text
);

create table Account (
  id            serial    primary key,
  password_hash bytea     not null,
  password_salt bytea     not null,
  email         text      not null,
  session_token text,
  expiration    timestamp
);

create table Registration (
  token         text  not null,
  steam_id      text  not null,
  name          text  not null,
  email         text  not null,
  player_tag    text,
  rank          text,
  password_hash bytea not null,
  password_salt bytea not null
);
