-- Create subscriptions table.
create table subscriptions (
    id uuid not null default uuidv7(),
    primary key (id),

    email text not null unique,
    name text not null,
    subscribed_at timestamptz not null default current_timestamp
);
