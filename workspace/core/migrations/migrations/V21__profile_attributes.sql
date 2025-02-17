drop table profile_attributes;
drop table profile_attribute_types;

create table profile_attribute_types
(
    id          varchar            not null,
    name        varchar            not null,
    description varchar            not null,
    visibility  profile_visibility not null default 'system'::profile_visibility,
    primary key (id)
);

create table profile_attributes
(
    id         uuid               not null default gen_random_uuid(),
    profile    uuid               not null,
    type_id    varchar            not null,
    visibility profile_visibility not null default 'system'::profile_visibility,
    attributes jsonb              not null,
    confidence int                not null,
    priority   int                not null,
    source     varchar            not null,
    created    timestamp          not null default now(),
    expiration timestamp,
    foreign key (profile) references profiles (id) on delete cascade,
    foreign key (type_id) references profile_attribute_types (id) on delete cascade,
    primary key (id)
);

alter table profiles
    add column collection_id uuid;
alter table profiles
    add foreign key (collection_id) references collections (id);

alter table principals
    add column verification_token varchar;

create unique index principals_verification_token on principals (verification_token) where verification_token is not null;

insert into traits (id, name, description, delete_workflow_id) values ('profile', 'Profile', 'User Profiles', null);

insert into profile_attribute_types (id, name, description, visibility)
values ('bosca.profiles.name', 'Name', 'Name', 'user'::profile_visibility);

insert into profile_attribute_types (id, name, description, visibility)
values ('bosca.profiles.email', 'Email', 'Email Address', 'user'::profile_visibility);

alter type collection_type add value 'system' after 'root';
