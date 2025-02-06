create table metadata_workflow_schedules
(
    metadata_id    uuid                     not null,
    workflow_id    varchar                  not null,
    attributes     jsonb,
    configuration  jsonb                    not null,
    rrule          varchar                  not null,
    starts         timestamp with time zone not null,
    ends           timestamp with time zone not null,
    last_scheduled timestamp with time zone not null,
    last_run       timestamp with time zone not null,
    next_run       timestamp with time zone not null,
    enabled        boolean                  not null default true,
    primary key (metadata_id, workflow_id),
    foreign key (metadata_id) references metadata (id) on delete cascade,
    foreign key (workflow_id) references workflows (id) on delete cascade
);

create table collection_workflow_schedules
(
    collection_id  uuid                     not null,
    workflow_id    varchar                  not null,
    attributes     jsonb,
    configuration  jsonb                    not null,
    rrule          varchar                  not null,
    starts         timestamp with time zone not null,
    ends           timestamp with time zone not null,
    last_scheduled timestamp with time zone not null,
    last_run       timestamp with time zone not null,
    next_run       timestamp with time zone not null,
    enabled        boolean                  not null default true,
    primary key (collection_id, workflow_id),
    foreign key (collection_id) references collections (id) on delete cascade,
    foreign key (workflow_id) references workflows (id) on delete cascade
);