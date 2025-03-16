create type activity_parameter_scope as enum ('plan', 'content');

create table collection_supplementary
(
    collection_id     uuid                     not null,
    key               varchar                  not null,
    plan_id           uuid,
    name              varchar                  not null,
    content_type      varchar                  not null,
    content_length    bigint,
    created           timestamp with time zone not null default now(),
    modified          timestamp with time zone not null default now(),
    source_id         uuid,
    source_identifier varchar,
    uploaded          timestamp with time zone,
    attributes        jsonb,
    primary key (collection_id, key, plan_id),
    foreign key (collection_id) references collections (id) on delete cascade,
    foreign key (source_id) references sources (id)
);

alter table metadata_supplementary
    drop constraint metadata_supplementary_pkey cascade;
alter table metadata_supplementary
    add column plan_id uuid;
alter table metadata_supplementary_traits
    add column plan_id uuid;
alter table metadata_supplementary
    add primary key (metadata_id, key, plan_id);
alter table metadata_supplementary_traits
    add foreign key (metadata_id, key, plan_id) references metadata_supplementary (metadata_id, key, plan_id) on delete cascade;

create table collection_supplementary_traits
(
    collection_id uuid    not null,
    key           varchar not null,
    plan_id       uuid,
    trait_id      varchar not null,
    primary key (collection_id, key, trait_id),
    foreign key (collection_id, key, plan_id) references collection_supplementary (collection_id, key, plan_id) on delete cascade,
    foreign key (trait_id) references traits (id) on delete cascade
);

alter table collections
    add column public_supplementary boolean not null default false;

alter table activity_outputs add column scope activity_parameter_scope not null default 'content';
alter table workflow_activity_outputs add column scope activity_parameter_scope not null default 'content';