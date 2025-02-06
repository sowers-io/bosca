drop table profile_attributes;
create table profile_attributes
(
    id         uuid               not null,
    profile    uuid               not null,
    type_id    uuid               not null,
    visibility profile_visibility not null default 'system'::profile_visibility,
    attributes jsonb              not null,
    confidence int                not null,
    priority   int                not null,
    source     varchar            not null,
    created    timestamp          not null default now(),
    expiration timestamp,
    primary key (id),
    foreign key (profile) references profiles (id) on delete cascade,
    foreign key (type_id) references profile_attribute_types (id) on delete cascade
);

alter table principals
    add column verification_token varchar;

create index principals_verification_token on principals (verification_token) where verification_token is not null;