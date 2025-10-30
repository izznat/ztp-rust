-- Create subscriptions table.
create table subscriptions (
    id text not null default (uuid7()),
    email text not null unique,
    name text not null,
    subscribed_at text not null default current_timestamp,

    primary key (id)
);
