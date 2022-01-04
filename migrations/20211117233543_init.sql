-- Add migration script here

create schema if not exists linkclub;

/* A linkclub user */
create table linkclub.app_user(
    id uuid,
    time_created timestamptz not null,
    username text not null,
    email text not null,

    primary key(id),
    unique(username)
);

/* A linkclub community */
create table linkclub.community(
    id uuid,
    time_created timestamptz not null,
    title text not null,

    primary key(id),
    unique(title)
);

/*
* Many-to-many relation for keeping track of users and
* what communities they belong to
*/
create table linkclub.community_members(
    id uuid,
    date_joined timestamptz not null,
    app_user uuid not null,
    community uuid not null,

    constraint fk_app_user foreign key(
        app_user
    ) references linkclub.app_user(id),
    constraint fk_community foreign key(
        community
    ) references linkclub.community(id),

    unique(app_user, community),
    primary key(id)
);

/* A url that has been posted to linkclub */
create table linkclub.url(
    id uuid,
    url text not null,

    primary key(id),
    unique(url)
);

/*
* All the promotes that have been done for a link.
* Each link is posted by a user, and belongs to a community
*/
create table linkclub.promote(
    id uuid,
    url uuid not null,
    app_user uuid not null,
    community uuid not null,
    time_promoted timestamptz not null,

    upvotes int not null,
    downvotes int not null,

    constraint fk_url foreign key(url) references linkclub.url(id),
    constraint fk_app_user foreign key(
        app_user
    ) references linkclub.app_user(id),

    primary key(id)
);
