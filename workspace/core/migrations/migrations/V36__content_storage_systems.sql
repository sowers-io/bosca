create table metadata_storage_systems
(
    id                uuid not null,
    storage_system_id uuid not null,
    primary key (id, storage_system_id),
    foreign key (id) references metadata (id) on delete cascade,
    foreign key (storage_system_id) references storage_systems (id) on delete cascade
);

create table collection_storage_systems
(
    id                uuid not null,
    storage_system_id uuid not null,
    primary key (id, storage_system_id),
    foreign key (id) references collections (id) on delete cascade,
    foreign key (storage_system_id) references storage_systems (id) on delete cascade
);