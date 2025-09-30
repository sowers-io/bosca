create type comment_status as enum ('pending', 'blocked', 'pending_approval', 'approved');

create table comments
(
    parent_id         bigint,
    id                bigserial,
    metadata_id       uuid,
    version           int,
    collection_id     uuid,
    profile_id        uuid not null,
    visibility        profile_visibility       default 'user',
    created           timestamp with time zone default now(),
    modified          timestamp with time zone default now(),
    status            comment_status           default 'pending'::comment_status,
    content           text not null check (length(content) > 0),
    attributes        jsonb,
    system_attributes jsonb,
    deleted           boolean                  default false,
    primary key (id),
    foreign key (metadata_id) references metadata (id),
    foreign key (collection_id) references collections (id),
    foreign key (profile_id) references profiles (id),
    foreign key (parent_id) references comments (id),
    check ( (metadata_id is not null and version is not null) or collection_id is not null )
);

create index comment_metadata_ix on comments (metadata_id, version, visibility, status, created desc) where metadata_id is not null and status != 'pending';
create index comment_collection_ix on comments (collection_id, visibility, status, created desc) where collection_id is not null and status != 'pending';